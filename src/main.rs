use eframe::egui;


struct Field {
    name: String,
    value: String,
}

struct Block {
    title: String,
    fields: Vec<Field>,
}

#[derive(Default)]
struct App {
    blocks: Vec<Block>,
    website_name: String,
    new_title: String,
    new_field_name: String,
    new_field_value: String,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            title: "Новый раздел".to_string(),
            fields: vec![Field {
                name: "Поле".to_string(),
                value: "Значение".to_string(),
            }],
        }
    }
}

impl App {
    fn save_html(&self) -> String{
        let mut html = String::from(r#"
<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Сгенерированный сайт</title>
    <style>
        :root {
            --blood: #8B0000;
            --dark: #000000;
            --text: #ffffff;
            --block-bg: #121212;
            --border: #333333;
        }
        * { 
            box-sizing: border-box; 
            margin: 0;
            padding: 0;
        }
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            line-height: 1.6;
            color: var(--text);
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background-color: var(--dark);
            min-height: 100vh;
        }
        
        .header-container {
            display: flex;
            gap: 30px;
            margin-bottom: 30px;
            align-items: center;
        }
        
        .ascii-art {
            font-family: monospace;
            font-size: 10px;
            line-height: 1;
            white-space: pre;
            color: var(--blood);
            width: 250px;
            flex-shrink: 0;
        }
        
        .info-container {
            flex: 1;
            padding-left: 500px;
            padding-top: 10px;
            padding-bottom: 10px;
        }
        
        .info-container h2 {
            color: var(--blood);
            margin-bottom: 15px;
            font-size: 1.8rem;
            text-shadow: 0 0 5px rgba(139, 0, 0, 0.3);
        }
        
        .info-text {
            color: var(--text);
            line-height: 1.7;
            margin-bottom: 20px;
            font-size: 1.1rem;
        }
        
        .info-links {
            display: flex;
            gap: 25px;
            padding-top: 15px;
            border-top: 1px solid var(--border);
        }
        
        .info-links a {
            color: var(--blood);
            text-decoration: none;
            font-weight: 600;
            font-size: 1.2rem;
            transition: all 0.3s;
            text-shadow: 0 0 3px rgba(139, 0, 0, 0.2);
        }
        
        .info-links a:hover {
            color: #ff3333;
            text-decoration: underline;
            transform: translateY(-2px);
        }
        
        .block {
            background: var(--block-bg);
            border-radius: 10px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
            margin-bottom: 30px;
            overflow: hidden;
            transition: transform 0.3s ease, box-shadow 0.3s ease;
            border: 1px solid var(--border);
        }
        .block:hover {
            transform: translateY(-5px);
            box-shadow: 0 8px 20px rgba(0, 0, 0, 0.7);
        }
        .block-header {
            background: var(--dark);
            color: var(--blood);
            padding: 20px;
            font-size: 1.5rem;
            font-weight: 700;
            text-transform: uppercase;
            letter-spacing: 1px;
            border-bottom: 2px solid var(--blood);
            text-shadow: 0 0 5px rgba(139, 0, 0, 0.3);
        }
        .block-content {
            padding: 25px;
        }
        .field {
            display: flex;
            border-bottom: 1px solid var(--border);
            padding: 15px 0;
        }
        .field:last-child {
            border-bottom: none;
        }
        .field-name {
            font-weight: 600;
            color: var(--blood);
            min-width: 200px;
            padding-right: 20px;
            text-shadow: 0 0 3px rgba(139, 0, 0, 0.2);
        }
        .field-value {
            flex-grow: 1;
            color: var(--text);
        }
        .add-block-btn {
            display: block;
            width: 100%;
            text-align: center;
            background: var(--blood);
            color: var(--text);
            padding: 15px;
            border-radius: 8px;
            cursor: pointer;
            font-weight: 600;
            margin-top: 20px;
            transition: all 0.3s;
            border: none;
            font-size: 1.1rem;
            letter-spacing: 0.5px;
            box-shadow: 0 4px 0 rgba(0, 0, 0, 0.3);
        }
        .add-block-btn:hover {
            background: #6B0000;
            transform: translateY(-2px);
            box-shadow: 0 6px 0 rgba(0, 0, 0, 0.4);
        }
        .add-block-btn:active {
            transform: translateY(1px);
            box-shadow: 0 2px 0 rgba(0, 0, 0, 0.4);
        }
        @media (max-width: 768px) {
            .header-container {
                flex-direction: column;
            }
            
            .ascii-art {
                width: 100%;
                font-size: 2px;
                margin-bottom: 15px;
            }
            
            .info-container {
                border-left: none;
                border-top: 2px solid var(--blood);
                padding-top: 15px;
                padding-left: 0;
            }
        }
    </style>
</head>
<body>
    <div class="header-container">
        <pre class="ascii-art">@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%@@@@@@@@#=%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@=%@@@@@@@@@-:%@#==------==+*#%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%=-@@@@@@@@@@::+@%:---==---::::::=+#@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@#::%@@@@@@@@@+::+@@@@@@@@@@@@%#*+-::::=#@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@+::*@@@@@@@@%-:::#@@@@@@@@@@@@@@@@@@#+:::-*%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@+::=@@@@@@@#=::::*@@@@@@@@@%%%%%%%%%%%%%#=:::+%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@%:::#@@@@@%=:::::#@@@@@@@@@%%###########%@@%+:::*@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@*:::+@@@@%:::::-%@@@@@@@@@@%%###########%@@@@%=::-%@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@#::::*@@@-::::=@@@@@@@@@@@@%%###########%@@@@@@#-::*@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@-::+%@@*:::::#@@@@@@@@@@@@%%###########%@@@@@@@%=::+@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@%#@%++@@@@*:::::#@@@@@@@@@@@@%%###########%@@@@@@@@@+::+@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@-:*@@#*@@@%-:::::+%@@@@@@@@@@%%###########%@@@@@@@@@@*::+@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@+::#@@-:+%@@@#+++*%@@@@@@@@@@@@%###########%@@@@@@@@@@@+::#@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@#::*@@%::::-+#@@@@@@@@@@@@@@@@@%%###########%@@@@@@@@@@@@-::%@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@-:-@@@*::::-=--#@@@%#######################*############%%::+@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@*::#@@@+:=*%=::::#@@%%%%%################################%@+::%@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@=:-@@@@#-*@#*+-::+@@%%###################################%@%::*@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@%::+@@@@%::+%*-:::*@@%%%##################################%@@-:=@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@#::#@@@%-:::::::::#@@%%%%%######%##%%#####################%@@+::@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@*::%@@%=::::::::::%@@%%%%%%%%%%%%%%%%%%##%################%@@*::%@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@+::@@@+--:::::::::#@@%%%%%%%%%%%%%%%%%%##%###%%%%%%%%%%%%#%@@#::#@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@+::@@@@%=:::::::::+@@@@@@@@@@@@@@@%%%%%%%%%%#%%@@%%%%%%@@@%@@*::%@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@*::%@@@@*-:::::::::#@@@@@@@@@@@@@@%%%%%%%%%%##%@@@%%%%%@@@@@@*::%@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@#::*@@@@@*=:::::::::*@@@@@@@@@@@@@%%%%%%%%%%##%@@@@%@@%@@@@@@=:-@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@::=@@@@@+::::::::-::-*@@@@@@@@@@@%%%%%%%%%%##%@@@@%@@%@@@@@@-:+@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@+::%@@@@#::::::=##::::=@@@@@@@@@@%%%%%%%%%%##%@@@@%@%%@@@@@#::#@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@%::+@@@@%+==*#@@@=:::::%@@@@@@@@@%%%%%%%%%%#%%@@@@#@@@@@@@@-:-@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@+::%@@@@@@@@@@@@%:::::*@@@@@@@@@%%%%%%%%%%#%%@@@%#%@@@@@@*::#@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@%-:=@@@@@@@@@@@@@=::::=@@@@@@@@@%%%%%%%%%%#%%@@@%%%@@@@@%-:=@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@#::+@@@@@@@@@@@@*:::::%@@@@@@@@%%%%%%%%%%%%%@@@@@@@@@@%-:-%@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@*::+@@@@@@@@@@@#:::::-%@@@@@@@%%%%%%%%%%#%@@@@@@@@@@%=:-%@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@*::=@@@@@@@@@@+::::::-#@@@@@@%%%%%%%%##%%@@@@@@@@@%-:-%@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@#-:-%@@@@@@@#::::::::-%@@@@@%%%%%%%%%%*:=%@@@@@@#-:-%@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@%-::*@@@@%+::::::::+@@@@@@%%%%%%%%%@#-::-%@@@%+::=%@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@+::-#@%-:::::::+%@@@@@@@@@@%%%%%%@@%=--**%%-:-*@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%=::-*%#=:::-#@@@@@@@@@@@@@%%@%@@@@@%-::-##+%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@#=::-+#%#*@@@@@@@@@@@@@@@%@@%@@@@@%=::::*@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%+-::-+#%@@@@@@@@@@@@@@%@@%%%*=-#@*::::+@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@#+-:::-=+*##%%@@@%@%%@@%#@--+#@@#-:::=%@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%*+=-::::::::::%%%%@%%@@@@@@@@%-:::-#@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%#*******@%#%@@@@@@@@@@@@%=::::*@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%#%@@@@@@@@@@@@@@=::::=%@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%#*@@@@@@@@@@@@@@@+::::-%@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%#+%@@@@@@@@@@@@@@@*::::=@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%#*#@@@@@@@@@@@@@@@@#=-=%@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%%%%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@</pre>
        
        <div class="info-container">
            <h2>О проекте</h2>
            <p class="info-text">Меня зовут Morph. Я занимаюсь OSINT — разведкой на основе открытых источников. Моя специализация охватывает не только классический OSINT, но и его смежные направления: анализ социальных сетей (SOCMINT), геолокацию (GEOINT), поиск по утечкам, цифровую атрибуцию и информационную безопасность.</p>
            
            <p class="info-text">Я основал проект "Кровавый Крест" — инициативу, направленную на борьбу с самыми тёмными сторонами интернета. Цель проекта — поиск и разоблачение тех, кто причастен к насилию над людьми и животными: живодёров, распространителей материалов с участием несовершеннолетних, зоофилов и других преступных элементов.</p>
            
            <p class="info-text">В рамках проекта мы используем анализ цифровых следов, карты, поведенческие шаблоны и общедоступные базы, чтобы выявлять личности, связанные с подобной деятельностью, и передавать информацию в соответствующие инстанции.</p>
            
            <div class="info-links">
                <a href="">Проект</a>
                <a href="">Предложки</a>
                <a href="">Владелец</a>
            </div>
        </div>
    </div>
    
        "#);
        for block in &self.blocks {
            html.push_str(&format!(
                r#"<div class="block">
                    <div class="block-header">{}</div>
                    <div class="block-content">"#,
                escape_html(&block.title)
            ));

            for field in &block.fields {
                html.push_str(&format!(
                    r#"<div class="field">
                        <div class="field-name">{}</div>
                        <div class="field-value">{}</div>
                    </div>"#,
                    escape_html(&field.name),
                    escape_html(&field.value)
                ));
            }

            html.push_str("</div></div>");
        }
        html.push_str("</body></html>");
        html
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Добро пожаловать в генератор сайтов! Ниже заполните все опции и сгенерируйте свой сайт");
            ui.separator();
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Название раздела: ");
                    ui.text_edit_singleline(&mut self.new_title);
                    ui.add_space(10.0);
                    ui.label("Добавить поле:");
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.new_field_name)
                            .on_hover_text("Название");
                        ui.text_edit_singleline(&mut self.new_field_value)
                            .on_hover_text("Значение");
                    });
                });
                if ui.button("Создать раздел").clicked() && !self.new_title.is_empty() {
                    self.blocks.push(Block {
                        title: self.new_title.clone(),
                        fields: if !self.new_field_name.is_empty() {
                            vec![Field {
                                name: self.new_field_name.clone(),
                                value: self.new_field_value.clone(),
                            }]
                        } else {
                            Vec::new()
                        },
                    });
                    self.new_title.clear();
                    self.new_field_name.clear();
                    self.new_field_value.clear();
                }
            });
            ui.separator();
            ui.heading("Разделы:");
            let mut block_to_remove = None;
            let mut field_to_remove: Option<(usize, usize)> = None;

            for (block_id, block) in self.blocks.iter_mut().enumerate() {
                ui.group(|ui| {
                    ui.label("Заголовок");
                    ui.text_edit_singleline(&mut block.title);

                    if ui.button("✕").on_hover_text("Удалить раздел").clicked() {
                        block_to_remove = Some(block_id);
                    }

                });
                ui.separator();
                for (field_id, field) in block.fields.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut field.name).on_hover_text("Поле");
                        ui.text_edit_singleline(&mut field.value).on_hover_text("Значение");
                        if ui.button("✕").on_hover_text("Удалить поле").clicked() {
                            field_to_remove = Some((block_id, field_id));
                        };
                    });
                };
                if ui.button("+ добавить поле").clicked() {
                    block.fields.push(Field { name: "Новое поле".to_string(), value: "Значение".to_string()});
                };
            };
            if let Some(id) = block_to_remove {
                self.blocks.remove(id);
            };
            if let Some((block_id, field_id)) = field_to_remove {
                self.blocks[block_id].fields.remove(field_id);
            }
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Имя сайта: ");
                ui.text_edit_singleline(&mut self.website_name);
            });
            if ui.button("Выгрузить в .html").on_hover_text("Создастся html файл в той же папке, где приложение").clicked() {
                let html = App::save_html(&self);
                let html_name = format!("{}.html", self.website_name);
                if let Err(e) = std::fs::write(html_name, html) {
                    ui.label(format!("Ошибка сохранения файла: {}", e.to_string()));
                    log::error!("{}", e.to_string());
                } else {
                    ui.label("Файл успешно сохранён");
                }
            };
        });
    }
}

fn escape_html(s: &str) -> String {
    ammonia::clean(s)
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Конструктор информационных блоков",
        options,
        Box::new(|_| Ok(Box::<App>::default())),
    );
}