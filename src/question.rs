use crate::*;

#[derive(Properties,Clone,PartialEq, serde::Deserialize)]
pub struct QuestionStruct {
    pub id: String,
    pub title: String,
    pub content: String,
    //pub tags: String,
    pub tags: Option<HashSet<String>>,
}

impl QuestionStruct{
    pub async fn get_question(key: Option<String>) -> Msg{
        let host = include_str!("../api-url.txt").trim();
        let request = match &key{
            None => format!(
                "{}/api/v1/question",
                host,
            ),
            Some(ref key) => format!(
                "{}/api/vi/joke/{}",
                host,
                key,
            ),
            };
            let response = http::Request::get(&request).send().await;
            match response {
                Err(e) => Msg::GotQuestion(Err(e)),
                Ok(data) => Msg::GotQuestion(data.json().await),
            }
        }
    }


#[derive(Properties,Clone,PartialEq,serde::Deserialize)]
pub struct QuestionProps{
    pub question: QuestionStruct,
}

pub fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(",")
}

#[function_component(Question)]
pub fn question(question: &QuestionProps) -> Html {
    html! { <>
        <div class = "question">
            <span>{"QandA!"}</span><br/>
            <span>{question.question.title.clone()}</span><br/>
            <span>{question.question.content.clone()}</span><br/>
        </div>
    </>}
}