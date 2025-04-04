/*
 * This file is part of Cockpit.
 *
 * Copyright (C) 2024 Red Hat, Inc.
 *
 * Cockpit is free software; you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation; either version 2.1 of the License, or
 * (at your option) any later version.
 *
 * Cockpit is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with Cockpit; If not, see <https://www.gnu.org/licenses/>.
 */

// @cockpit-ts-relaxed

import cockpit from "cockpit";

import React from 'react';
import ReactDOM from "react-dom";
import { Button } from "@patternfly/react-core/dist/esm/components/Button";
import {
    CaretDownIcon,
    CaretUpIcon,
    EditIcon,
    MinusIcon,
} from '@patternfly/react-icons';
import { PageSidebar } from "@patternfly/react-core/dist/esm/components/Page";
import { Tooltip } from "@patternfly/react-core/dist/esm/components/Tooltip";

import 'polyfills';
import { CockpitNav, CockpitNavItem } from "./nav.jsx";
import { encode_location } from "./util.jsx";
import { split_connection_string } from "./machines/machines";
import { add_host, edit_host, connect_host } from "./hosts_dialog.jsx";

import { ShellState } from "./state";
import { Icon } from "@patternfly/react-core";

const _ = cockpit.gettext;

class HostsSelector extends React.Component {
    el: HTMLDivElement;
    props: { children };

    constructor(props) {
        super(props);
        this.props = props;

        this.el = document.createElement("div");
        this.el.className = "view-hosts";
    }

    componentDidMount() {
        const hosts_sel = document.getElementById("nav-hosts");
        hosts_sel?.appendChild(this.el);
    }

    componentWillUnmount() {
        const hosts_sel = document.getElementById("nav-hosts");
        hosts_sel?.removeChild(this.el);
    }

    render() {
        const { children } = this.props;
        return ReactDOM.createPortal(children, this.el);
    }
}

function HostLine({ host, user }) {
    return (
        <>
            <span id="current-username" className="username">{user}</span>
            {user && <span className="at">@</span>}
            <span className="hostname">{host}</span>
        </>
    );
}

// top left navigation element when host switching is disabled
export const CockpitCurrentHost = ({ current_user, machine }) => {
    return (
        <div className="ct-switcher ct-switcher-localonly pf-m-dark">
            <HostLine user={machine.user || current_user || ""} host={machine.label || ""} />
        </div>
    );
};

interface CockpitHostsState { opened, editing, current_user, current_key }

// full host switcher
export class CockpitHosts extends React.Component {
    props: { selector, host_modal_state, state: ShellState };
    state: CockpitHostsState;

    constructor(props) {
        super(props);
        this.props = props;

        this.state = {
            opened: false,
            editing: false,
            current_user: "",
            current_key: props.state.current_machine.key,
        };

        this.toggleMenu = this.toggleMenu.bind(this);
        this.filterHosts = this.filterHosts.bind(this);
        this.onAddNewHost = this.onAddNewHost.bind(this);
        this.onEditHosts = this.onEditHosts.bind(this);
        this.onHostEdit = this.onHostEdit.bind(this);
        this.onRemove = this.onRemove.bind(this);
    }

    componentDidMount() {
        cockpit.user().then(user => {
            this.setState({ current_user: user.name || "" });
        }).catch(exc => console.log(exc));
    }

    static getDerivedStateFromProps(nextProps, prevState) {
        if (nextProps.state.current_machine.key !== prevState.current_key) {
            document.getElementById(nextProps.selector)?.classList.toggle("interact", false);
            return {
                current_key: nextProps.state.current_machine.key,
                opened: false,
                editing: false,
            };
        }
        return null;
    }

    toggleMenu() {
        document.getElementById(this.props.selector)?.classList.toggle("interact", !this.state.opened);

        this.setState((s: CockpitHostsState) => {
            return (
                {
                    opened: !s.opened,
                    editing: false,
                }
            );
        });
    }

    async onAddNewHost() {
        await add_host(this.props.host_modal_state, this.props.state);
    }

    async onHostEdit(_event, machine) {
        await edit_host(this.props.host_modal_state, this.props.state, machine);
    }

    async onHostSwitch(machine) {
        const { state, host_modal_state } = this.props;

        const connection_string = await connect_host(host_modal_state, state, machine);
        if (connection_string) {
            const parts = split_connection_string(connection_string);
            state.jump({ host: parts.address });
        }
    }

    onEditHosts() {
        this.setState((s: CockpitHostsState) => { return { editing: !s.editing } });
    }

    onRemove(event, machine) {
        const { state } = this.props;
        const { current_machine } = state;

        event.preventDefault();

        if (current_machine === machine) {
            // Removing machine underneath ourself - jump to localhost
            state.jump({ host: "localhost" });
        }

        if (state.machines.list.length <= 2)
            this.setState({ editing: false });
        state.machines.change(machine.key, { visible: false });
    }

    filterHosts(host, term) {
        if (!term)
            return host;
        const new_host = Object.assign({}, host);
        term = term.toLowerCase();

        if (host.label.toLowerCase().indexOf(term) > -1)
            new_host.keyword = host.label.toLowerCase();

        const user = host.user || this.state.current_user;
        if (user.toLowerCase().indexOf(term) > -1)
            new_host.keyword = user.toLowerCase() + " @";

        if (new_host.keyword)
            return new_host;
        return null;
    }

    // HACK: using HTML rather than Select PF4 component as:
    // 1. It does not change the arrow when opened/closed
    // 2. It closes the dropdown even when trying to search... and cannot tell it not to
    render() {
        const { state } = this.props;
        const { current_machine } = state;

        const editing = this.state.editing;
        const groups = [{
            name: _("Hosts"),
            items: state.machines.list,
        }];
        const render = (m, term) => <CockpitNavItem
                term={term}
                keyword={m.keyword}
                href={encode_location({ host: m.address })}
                active={m === current_machine}
                key={m.key}
                name={m.label}
                header={(m.user ? m.user : this.state.current_user) + " @"}
                status={m.state === "failed" ? { type: "error", title: _("Connection error") } : null}
                className={m.state}
                onClick={() => this.onHostSwitch(m)}
                actions={<>
                    <Tooltip content={_("Edit")} position="right">
                        <Button isDisabled={m.address === "localhost"} className="nav-action" hidden={!editing} onClick={e => this.onHostEdit(e, m)} key={m.label + "edit"} variant="secondary"><EditIcon /></Button>
                    </Tooltip>
                    <Tooltip content={_("Remove")} position="right">
                        <Button isDisabled={m.address === "localhost"} onClick={e => this.onRemove(e, m)} className="nav-action" hidden={!editing} key={m.label + "remove"} variant="danger"><MinusIcon /></Button>
                    </Tooltip>
                </>}
        />;
        const label = current_machine?.label || "";
        const user = current_machine?.user || this.state.current_user;

        const add_host_action = <Button variant="secondary" onClick={this.onAddNewHost}>{_("Add new host")}</Button>;

        return (
            <div className="ct-switcher">
                <div className="pf-v6-c-select pf-m-dark">
                    <button onClick={this.toggleMenu} id="host-toggle" aria-labelledby="host-toggle" aria-expanded={(this.state.opened ? "true" : "false")} aria-haspopup="listbox" type="button" className="ct-nav-toggle pf-v6-c-select__toggle pf-v6-c-menu-toggle pf-m-plain">
                        <span className="pf-v6-c-button__text desktop_v">
                            <span className="pf-v6-c-select__toggle-wrapper">
                                <span className="pf-v6-c-select__toggle-text">
                                    <HostLine user={user} host={label} />
                                </span>
                            </span>
                        </span>
                        <Icon size="xl" className="mobile_v">
                            <CaretUpIcon
                                className={`pf-v6-c-select__toggle-arrow ${this.state.opened ? 'clicked' : ''}`}
                                aria-hidden="true"
                            />
                        </Icon>
                        <span className="pf-v6-c-select__toggle-wrapper mobile_v">
                            {_("Host")}
                        </span>
                        <Icon size="xl" className="desktop_v">
                            <CaretDownIcon
                                className={`pf-v6-c-select__toggle-arrow ${this.state.opened ? 'clicked' : ''}`}
                                aria-hidden="true"
                            />
                        </Icon>
                    </button>
                </div>

                { this.state.opened &&
                <HostsSelector>
                    <PageSidebar className={"sidebar-hosts" + (this.state.editing ? " edit-hosts" : "")}>
                        <CockpitNav
                            selector={this.props.selector}
                            groups={groups}
                            item_render={render}
                            sorting={(_a, _b) => 1}
                            filtering={this.filterHosts}
                            current={label}
                            jump={() => console.error("internal error: jump not supported in hosts selector")}
                        />
                        <div className="nav-hosts-actions">
                            {state.machines.list.length > 1 && <Button variant="secondary" onClick={this.onEditHosts}>{this.state.editing ? _("Stop editing hosts") : _("Edit hosts")}</Button>}
                            {add_host_action}
                        </div>
                    </PageSidebar>
                </HostsSelector>
                }
            </div>
        );
    }
}
