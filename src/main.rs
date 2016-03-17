extern crate regex;

use regex::Regex;


struct Text<'a> {
    origin: &'a str,
    temp: String
}

impl<'a> Text<'a> {
    fn canonize(&mut self) {
        let html = Regex::new(r"<[^>]*>|[:punct:]").unwrap();
        self.temp = html.replace_all(self.origin, " ");        
    }
}


static TEXT1: &'static str = "Сегодня, 27 октября, на сессии Госсовета Удмуртии мандат Дмитрия Кулишова, который сложил полномочия для того, чтобы перейти в Городскую Думу Ижевска, был передан Геннадию Зарубежнову – «зарегистрированному кандидату из республиканского списка кандидатов выдвинутого избирательным объединением удмуртское региональное отделение политической партии ЛДПР». По данным ЦИК УР, Геннадий Зарубежнов (1962 г.р.) является преподавателем ОУ ССПО «Нефтяной техникум», а также депутатов Совета депутатов Завьяловского района на непостоянной основе. На сайте Завьяловского района также указано, что он работает заместителем директора группы компаний «Прогресс».";
static TEXT2: &'static str = "Геннадий Зарубежнов стал депутатом Госсовета Удмуртии. Он получил мандат, освободившийся после перехода руководителя фракции ЛДПР Дмитрия Кулишова в Гордуму Ижевска. Зарубежнов шёл по спискам ЛДПР на выборах 2012 года. Сегодня на сессии республиканского парламента ему вручили удостоверение и знак депутата. Геннадий Зарубежнов занимает пост директора фирмы Прогресс, до недавнего времени являлся депутатом совета депутатов Завьяловского район";

fn main() {
    let mut first = Text {
        origin: TEXT1,
        temp: String::new()
    };

    first.canonize();

    println!("{}", first.temp);

}