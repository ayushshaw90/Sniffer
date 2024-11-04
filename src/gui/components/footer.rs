//! GUI bottom footer

use std::sync::{Arc, Mutex};

use iced::alignment::{Horizontal, Vertical};
// use iced::widget::text::LineHeight;
// use iced::widget::tooltip::Position;
use iced::widget::{button, Container, Row, Text, Tooltip};
use iced::widget::{horizontal_space, Space};
use iced::{Alignment, Font, Length};

// use crate::gui::components::button::row_open_link_tooltip;
// use crate::gui::styles::button::ButtonType;
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::{FONT_SIZE_FOOTER, FONT_SIZE_SUBTITLE};
// use crate::gui::styles::text::TextType;
use crate::gui::styles::types::gradient_type::GradientType;
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
// use crate::translations::translations_2::new_version_available_translation;
// use crate::utils::formatted_strings::APP_VERSION;
// use crate::utils::types::icon::Icon;
// use crate::utils::types::web_page::WebPage;
use crate::{Language, SNIFFNET_TITLECASE};

pub fn footer(
    thumbnail: bool,
    language: Language,
    color_gradient: GradientType,
    font: Font,
    font_footer: Font,
    newer_release_available: &Arc<Mutex<Option<bool>>>,
) -> Container<'static, Message, StyleType> {
    if thumbnail {
        return thumbnail_footer();
    }

    // let release_details_row =
    //     get_release_details(language, font, font_footer, newer_release_available);

    let footer_row = Row::new()
        .spacing(10)
        .padding([0, 20])
        .align_items(Alignment::Center)
        // .push(release_details_row)
        // .push(get_button_website(font))
        // .push(get_button_github(font))
        // .push(get_button_sponsor(font))
        .push(
            Text::new("Made with ❤ by NIT Rourkela")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
                .size(FONT_SIZE_FOOTER)
                .font(font_footer),
        );

    Container::new(footer_row)
        .height(45)
        .align_y(Vertical::Center)
        .style(ContainerType::Gradient(color_gradient))
}



fn thumbnail_footer() -> Container<'static, Message, StyleType> {
    Container::new(horizontal_space()).height(0)
}
