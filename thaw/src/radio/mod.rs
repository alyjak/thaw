mod radio_group;

pub use radio_group::RadioGroup;

use leptos::prelude::*;
use radio_group::RadioGroupInjection;
use thaw_utils::{class_list, mount_style, OptionalProp};

#[component]
pub fn Radio(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] value: String,
    #[prop(optional, into)] label: MaybeProp<String>,
) -> impl IntoView {
    mount_style("radio", include_str!("./radio.css"));

    let id = uuid::Uuid::new_v4().to_string();
    let group = RadioGroupInjection::use_();
    let item_value = StoredValue::new(value);

    let checked = Memo::new({
        let group = group.clone();
        move |_| {
            item_value.with_value(|value| {
                group
                    .value
                    .with(|group_value| group_value.as_ref() == Some(value))
            })
        }
    });

    let on_change = move |_| {
        group.value.set(Some(item_value.get_value()));
    };

    view! {
        <span
            class=class_list![
                "thaw-radio", class.map(| c | move || c.get())
            ]
        >
            <input
                class="thaw-radio__input"
                type="radio"
                id=id.clone()
                name=group.name
                value=item_value.get_value()
                prop:checked=move || checked.get()
                on:change=on_change
            />
            <div aria-hidden="true" class="thaw-radio__indicator"></div>
            {
                move || if let Some(label) = label.get() {
                    view! {
                        <label class="thaw-radio__label" for=id.clone()>{label}</label>
                    }.into()
                } else {
                    None
                }
            }
        </span>
    }
}
