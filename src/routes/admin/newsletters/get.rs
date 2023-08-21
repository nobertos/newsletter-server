use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;

use std::fmt::Write;

pub async fn send_newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut error_html = String::new();
    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap()
    }
    HttpResponse::Ok().content_type(ContentType::html()).body(
    format!(    r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>Send newsletter</title>
    </head>
    <body>
        {error_html}
        <form action="/admin/newsletters" method="post">
            <div>
                <label>Title
                    <input type="text" placeholder="Enter title" name="title">
                </label>
            </div>
            <div>
                <label>Text content
                    <textarea placeholder="Enter your text body content." rows="20" cols="50" name="text_content">
                    </textarea>
                </label>
            </div>
            <div>
                <label>Html content
                    <textarea placeholder="Enter your Html body content." rows="20" cols="50" name="html_content">
                    </textarea>
                </label>
            </div>
            <br>
            <button type="submit">Publish</button>
        </form>
        <p><a href="/admin/dashboard">&lt;- Back</a></p>
    </body>
</html>

        "#),
    )
}
