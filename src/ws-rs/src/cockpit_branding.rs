use std::path::Path;

use actix_files::NamedFile;
use actix_web::{get, Error, HttpRequest, HttpResponse};
use std::{fs, io};

use crate::state::WebCockpitState;

fn find_file_from_roots(path: &str, roots: &Vec<String>) -> Result<NamedFile, Error> {
    // TODO: Handle root espace like (src/common/cockpitwebresponse.c:web_response_file)
    for root in roots {
        let combined = format!("{root}/{path}");
        let full_path = Path::new(&combined);
        if full_path.exists() {
            return NamedFile::open(full_path).map_err(|e| e.into());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Provided path doesn't exsist in roots",
    )
    .into())
}

fn replace_templates(base: &str, state: &WebCockpitState) -> String {
    // TODO: actually parse this src/common/cockpitwebresponse.c:web_response_file (cockpit_template_expand)
    base.replace("${NAME}", state.os_release().get("NAME").unwrap())
}

fn get_branding_css_file(path: &str, state: &WebCockpitState) -> Result<NamedFile, Error> {
    find_file_from_roots(path, state.branding())
}

// TODO: sepearate logic for static files (that can be cached) and css that's modified on every request
/* #[get("/cockpit/static/{filename}")]
async fn cockpit_static(state: WebCockpitState, req: HttpRequest) -> Result<NamedFile, Error> {
    let path/* : std::path::PathBuf */ = req.match_info().query("filename");// .parse().unwrap();
    let file = if path.ends_with(".css") {
        get_branding_css_file(path, &state)?
    } else {
        // NamedFile::open(&format!("{STATIC_BASE_PATH}{path}"/* , path.to_str().unwrap() */))?
        find_file_from_roots(path, state.branding())?
    };
    // let file = get_branding_css_file(&state)?;
    // let file = NamedFile::open(&format!("{STATIC_BASE_PATH}{path}"/* , path.to_str().unwrap() */))?;
    // if path.contains("branding") {
    //     println!("Request Branding: {path:?}");
    //     println!("Opened Branding: {file:?}");
    // }
    // if path.ends_with(".css") {
    //     println!("Request file: {path:?}");
    //     println!("Opened file: {file:?}");
    // }

    // TOOD: cockpit_auth_parse_application part in src/ws/cockpitbranding.c:cockpit_branding_serve

    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
} */

#[get("/cockpit/static/{filename}")]
async fn cockpit_static(state: WebCockpitState, req: HttpRequest) -> Result<HttpResponse, Error> {
    let path = req.match_info().query("filename");
    if path.ends_with(".jpg") {
        let content = fs::read(find_file_from_roots(path, state.branding())?.path())?;
        return Ok(HttpResponse::Ok().body(content));
    }

    let content = if path.ends_with(".css") {
        let base_css = fs::read_to_string(get_branding_css_file(path, &state)?.path())?;
        replace_templates(&base_css, &state)
    } else {
        fs::read_to_string(find_file_from_roots(path, state.branding())?.path())?
    };

    Ok(HttpResponse::Ok().body(content))
}
