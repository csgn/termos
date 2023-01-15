mod termos {
    // VARIANTS
    pub enum Variant {
        Choice,
        Select,
    }

    // OPTION
    struct Option {
        key: String,
        default: bool,
    }

    // PROMPT
    pub struct Prompt {
        variant: Variant,
        question: String,
        pub(crate) answer: String,
        options: Vec<Option>,
    }

    impl Prompt {
        pub fn add_option(&mut self, key: String, default: bool) -> &mut Prompt {
            let mut o = Option { key, default };
            self.options.push(o);

            self
        }

        pub fn get_answer(&mut self) {
            let mut answer = String::new();
            println!(">>> ");
            std::io::stdin().read_line(&mut answer);

            self.answer = answer;
        }
    }


    // MENU
    pub struct Menu {
        pub(crate) prompts: Vec<Prompt>,
    }

    impl Default for Menu {
        fn default() -> Self {
            Self {
                prompts: vec![],
            }
        }
    }

    impl Menu {
        pub fn add_prompt(&mut self, question: String, variant: Variant) -> &mut Prompt {
            let mut p: Prompt = Prompt {
                variant,
                question,
                answer: "".to_string(),
                options: vec![],
            };

            self.prompts.push(p);
            self.prompts.last_mut().unwrap()
        }

        pub fn show(&mut self) {
            for p in self.prompts.iter_mut() {
                println!("{}", p.question);
                p.get_answer();
            }
        }
    }
}


fn main() {
    let mut menu = termos::Menu::default();

    menu.add_prompt("Lorem ipsum".to_string(), termos::Variant::Choice)
        .add_option("y".to_string(), true)
        .add_option("n".to_string(), false);

    menu.show();

    println!("{}", menu.prompts.get(0).unwrap().answer);


}
