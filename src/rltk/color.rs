#[derive(PartialEq, Copy, Clone)]
pub struct Color {
    pub r : f32,
    pub g : f32,
    pub b : f32
}

#[allow(dead_code)]
impl Color {
    pub fn new(r: f32, g:f32, b:f32) -> Color {
        return Color{r, g, b};
    }

    pub fn to_greyscale(&self) -> Color {
        let linear = (self.r * 0.2126) + (self.g * 0.7152) + (self.b * 0.0722);
        return Color::new(linear, linear, linear);
    }

    pub fn white() -> Color {
        return Color{r:1.0, g: 1.0, b:1.0};
    }

    pub fn black() -> Color {
        return Color{r:0.0, g: 0.0, b:0.0};
    }

    pub fn red() -> Color {
        return Color{r:1.0, g:0.0, b:0.0};
    }

    pub fn green() -> Color {
        return Color{r:0.0, g:1.0, b:0.0};
    }

    pub fn dark_green() -> Color {
        return Color{r:0.0, g:0.5, b:0.0};
    }

    pub fn grey() -> Color {
        return Color{r:0.5, g:0.5, b:0.5};
    }

    pub fn yellow() -> Color {
        return Color{r:1.0, g:1.0, b:0.0};
    }

    pub fn magenta() -> Color {
        return Color{r:1.0, g:0.0, b:1.0};
    }

    pub fn cyan() -> Color {
        return Color{r:0.0, g:1.0, b:1.0};
    }

    pub fn orange() -> Color { Color{r:1.0, g:0.64, b:0.0} }
    pub fn blue() -> Color { Color{r:0.0, g:0.0, b:1.0} }
}
