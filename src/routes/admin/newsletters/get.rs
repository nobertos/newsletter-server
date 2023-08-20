use actix_web::{http::header::ContentType, HttpResponse};

pub async fn send_newsletter_form() -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta http-equiv="content-type" content="text/html; charset=utf-8">
        <title>Send newsletter</title>
    </head>
    <body>
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
        </form>
    </body>
</html>

        "#,
    )
}
