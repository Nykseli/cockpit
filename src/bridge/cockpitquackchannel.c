/*
 * This file is part of Cockpit.
 *
 * Copyright (C) 2014 Red Hat, Inc.
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
 * along with Cockpit; If not, see <http://www.gnu.org/licenses/>.
 */

#include "config.h"

#include "cockpitquackchannel.h"

#include "rust/rust.h"

/**
 * CockpitQuackChannel:
 *
 * A #CockpitChannel that sends "Quack to everyone"
 *
 * The payload type for this channel is 'quack'.
 */

#define COCKPIT_QUACK_CHANNEL(o)    (G_TYPE_CHECK_INSTANCE_CAST ((o), COCKPIT_TYPE_QUACK_CHANNEL, CockpitQuackChannel))

typedef struct {
  CockpitChannel parent;
} CockpitQuackChannel;

typedef struct {
  CockpitChannelClass parent_class;
} CockpitQuackChannelClass;

G_DEFINE_TYPE (CockpitQuackChannel, cockpit_quack_channel, COCKPIT_TYPE_CHANNEL);

static void
cockpit_quack_channel_recv (CockpitChannel *channel,
                           GBytes *message)
{
  gchar* msg;
  size_t msg_len;
  g_debug ("received quack channel payload");

  msg = (gchar*) g_bytes_get_data (message, &msg_len);
  msg[msg_len] = '\0';
  g_message("WHat do quack?: '%s'", msg);
  if (g_str_equal (msg, "quack"))
    {
      GBytes* ret = g_bytes_new ("Quack quack", 11);
      cockpit_channel_send (channel, ret, FALSE);
    }
  else
    {
      // GBytes* ret = g_bytes_new ("Not very quack", 14);
      GBytes* ret = get_quack();
      cockpit_channel_send (channel, ret, FALSE);
    }

//   GBytes* foo = g_bytes_new ("Quack quack", 11);
//   cockpit_channel_send (channel, foo, FALSE);
  // TODO: this causes an error. Will it memleak?
  //g_free(foo);
}

static gboolean
cockpit_quack_channel_control (CockpitChannel *channel,
                              const gchar *command,
                              JsonObject *options)
{
  if (g_str_equal (command, "done"))
    {
      g_debug ("received quack channel done");
      cockpit_channel_control (channel, command, options);
      return TRUE;
    }

  return FALSE;
}

static void
cockpit_quack_channel_init (CockpitQuackChannel *self)
{

}

static void
cockpit_quack_channel_prepare (CockpitChannel *channel)
{
  COCKPIT_CHANNEL_CLASS (cockpit_quack_channel_parent_class)->prepare (channel);
  cockpit_channel_ready (channel, NULL);
}

static void
cockpit_quack_channel_class_init (CockpitQuackChannelClass *klass)
{
  CockpitChannelClass *channel_class = COCKPIT_CHANNEL_CLASS (klass);

  channel_class->prepare = cockpit_quack_channel_prepare;
  channel_class->control = cockpit_quack_channel_control;
  channel_class->recv = cockpit_quack_channel_recv;
}
