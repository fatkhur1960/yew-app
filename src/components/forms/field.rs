use yew::prelude::*;

use super::{Form, Model};

pub enum FieldMessage {
    OnInput(InputData),
}

fn default_text() -> String {
    String::from("text")
}

#[derive(Properties, PartialEq, Clone)]
pub struct FieldProperties<T: Model> {
    #[prop_or_else(default_text)]
    pub input_type: String,
    pub name: String,
    pub form: Form<T>,
    #[prop_or_else(String::new)]
    pub placeholder: String,
    #[prop_or_else(Callback::noop)]
    pub oninput: Callback<InputData>,
    #[prop_or_default]
    pub maxlength: Option<i32>,
}

pub struct Field<T: Model> {
    link: ComponentLink<Self>,
    pub input_type: String,
    pub name: String,
    pub form: Form<T>,
    pub placeholder: String,
    pub oninput: Callback<InputData>,
    pub maxlength: Option<i32>,
}

impl<T: Model> Field<T> {
    pub fn field_name(&self) -> &str {
        &self.name
    }

    pub fn class(&self) -> &'static str {
        let s = self.form.state();
        let field = s.field(&self.name);

        if field.dirty && field.valid {
            "form-control"
        } else if field.dirty {
            "form-control is-invalid"
        } else {
            "form-control"
        }
    }

    pub fn class2(&self) -> &'static str {
        let s = self.form.state();
        let field = s.field(&self.name);

        if field.dirty && field.valid {
            "field-length"
        } else if field.dirty {
            "field-length is-invalid"
        } else {
            "field-length"
        }
    }

    pub fn message(&self) -> String {
        self.form.field_message(&self.field_name())
    }

    pub fn valid(&self) -> bool {
        self.form.field_valid(&self.field_name())
    }

    pub fn dirty(&self) -> bool {
        self.form.state().field(&self.name).dirty
    }

    pub fn set_field(&mut self, name: &str, value: &str) {
        self.form.set_field_value(name, value)
    }
}

impl<T: Model> Component for Field<T> {
    type Message = FieldMessage;
    type Properties = FieldProperties<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut form_field = Self {
            link,
            input_type: String::from(props.input_type),
            name: String::from(props.name),
            form: props.form,
            placeholder: String::from(props.placeholder),
            oninput: props.oninput,
            maxlength: props.maxlength,
        };

        if form_field.input_type == "" {
            form_field.input_type = String::from("text");
        }

        form_field
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FieldMessage::OnInput(input_data) => {
                let mut state = self.form.state_mut();
                state.set_field_value(&self.name, &input_data.value);
                state.update_validation_field(&self.name);
                drop(state);

                self.oninput.emit(input_data);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let maxlength = self
            .maxlength
            .map(|a| format!("{}", a))
            .unwrap_or(String::new());
        let value = self.form.field_value(&self.name);
        html! {
            <div class="input">
                <input
                    class=self.class()
                    id=self.name
                    type=self.input_type
                    placeholder=self.placeholder
                    value=value
                    name=&self.name
                    oninput=self.link.callback(|e: InputData| FieldMessage::OnInput(e))
                    maxlength=maxlength
                />
                {
                    if self.maxlength.is_some() {
                        html !{
                            <div class=self.class2()>
                                <span>{format!("{}/{}",&value.len(),self.maxlength.unwrap())}</span>
                            </div>
                        }
                    } else{
                        html! {}
                    }
                }
            </div>
        }
    }
}
