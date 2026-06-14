use crate::color::Rgb8;

#[derive(Debug, Copy, Clone)]
pub enum Format {
    /// Portable Bit Map ASCII
    P1,
    /// Portable Gray Map ASCII
    P2,
    /// Portable Pixel Map ASCII
    P3,
    /// Portable Bit Map Binary
    P4,
    /// Portable Gray Map Binary
    P5,
    /// Portable Pixel Map Binary
    P6,
}
impl Format {
    pub const fn extension(self) -> &'static str {
        use Format::*;
        match self {
            P1 | P4 => "pbm",
            P2 | P5 => "pgm",
            P3 | P6 => "ppm",
        }
    }

    pub const fn variant_name(self) -> &'static str {
        use Format::*;
        match self {
            P1 => "P1",
            P2 => "P2",
            P3 => "P3",
            P4 => "P4",
            P5 => "P5",
            P6 => "P6",
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Header {
    pub format: Format,
    pub height: u32,
    pub width: u32,
    pub maximum: u32,
}
impl Header {
    pub fn to_bytes(&self) -> Vec<u8> {
        format!(
            "{}\n{} {}\n{}\n",
            self.format.variant_name(),
            self.width,
            self.height,
            self.maximum
        )
        .into_bytes()
    }
}

impl Rgb8 {
    /// Returns a [String] that has the color channel values as per [ppm spec](https://netpbm.sourceforge.net/doc/ppm.html)
    ///
    /// - There is a trailing space
    /// # Example
    /// ```rust
    /// use ray_tracing::color::Rgb8;
    /// let color = Rgb8 { red: 255, green: 255, blue: 0};
    /// let ppm = color.ppm_ascii();
    ///
    /// assert_eq!(ppm, "255 255 0 ");
    /// ```
    pub fn to_ppm_ascii(self) -> String {
        format!("{} {} {} ", self.red, self.green, self.blue)
    }

    /// Returns a [Vec] of [u8] that has the color channel values as per [ppm spec](https://netpbm.sourceforge.net/doc/ppm.html)
    /// # Example
    /// ```rust
    /// use ray_tracing::color::Rgb8;
    /// let color = Rgb8 { red: 255, green: 255, blue: 0};
    /// let ppm = color.ppm_binary();
    ///
    /// assert_eq!(ppm, [255, 255, 0]);
    /// ```
    pub fn to_ppm_binary(self) -> [u8; 3] {
        self.to_array()
    }
}
