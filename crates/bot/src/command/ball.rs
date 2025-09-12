use bot_lib::random_range;

use teloxide::prelude::Requester;
use teloxide::types::Message;
use teloxide::{Bot, RequestError};

fn get_random_answer() -> &'static str {
    match random_range(0..20) {
        0 => "Бесспорно",
        1 => "Это предрешено",
        2 => "Никаких сомнений",
        3 => "Определенно – «да»",
        4 => "Можешь быть уверен в этом",
        5 => "Мне кажется - «да»",
        6 => "Вероятнее всего",
        7 => "Хорошие перспективы",
        8 => "Да",
        9 => "Знаки говорят - да",
        10 => "Пока не ясно, попробуй еще раз",
        11 => "Спроси позже",
        12 => "Лучше не рассказывать тебе это сейчас",
        13 => "Сейчас нельзя предсказать",
        14 => "Сконцентрируйся и спроси снова",
        15 => "Даже не думай",
        16 => "Мой ответ - нет",
        17 => "Знаки говорят - нет",
        18 => "Перспективы не очень хорошие",
        _ => "Весьма сомнительно",
    }
}

pub async fn answer(bot: Bot, msg: Message) -> Result<(), RequestError> {
    bot.send_message(msg.chat.id, get_random_answer()).await?;
    Ok(())
}
