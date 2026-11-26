use components::{questionnaire::*, rsx};
use nsh_core::prelude::*;

pub fn example() -> impl IntoElement {
    const ITEMS: &[QuestionnaireItem] = &[QuestionnaireItem::new("topic", "What should we build?")
        .choices(&[QuestionnaireChoice::new("plan", "A plan")])];
    rsx! {
        <View>
            {Questionnaire::new(ITEMS)}
        </View>
    }
}
