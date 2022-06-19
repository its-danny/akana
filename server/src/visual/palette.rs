use yansi::Color;

macro_rules! spectrum {
    ($hex:expr) => { rgb_to_color(hex_to_rgb($hex))};
    ($($h:expr)+) => {[ $(spectrum!($h)),+ ]};
}

pub struct Palette {
    pub slate: [Color; 10],
    pub gray: [Color; 10],
    pub zinc: [Color; 10],
    pub neutral: [Color; 10],
    pub stone: [Color; 10],
    pub red: [Color; 10],
    pub orange: [Color; 10],
    pub amber: [Color; 10],
    pub yellow: [Color; 10],
    pub lime: [Color; 10],
    pub green: [Color; 10],
    pub emerald: [Color; 10],
    pub teal: [Color; 10],
    pub cyan: [Color; 10],
    pub sky: [Color; 10],
    pub blue: [Color; 10],
    pub indigo: [Color; 10],
    pub violet: [Color; 10],
    pub purple: [Color; 10],
    pub fuchsia: [Color; 10],
    pub pink: [Color; 10],
    pub rose: [Color; 10],
}

impl Default for Palette {
    // Colors taken from Tailwind
    // https://tailwindcss.com/docs/customizing-colors
    fn default() -> Self {
        Self {
            slate: spectrum![
                "F8FAFC" "F1F5F9" "E2E8F0" "CBD5E1" "94A3B8"
                "64748B" "475569" "334155" "1E293B" "0F172A"
            ],
            gray: spectrum![
                "F9FAFB2" "F3F4F6" "E5E7EB" "D1D5DB" "9CA3AF"
                "6B7280" "4B5563" "374151" "1F2937" "111827"
            ],
            zinc: spectrum![
                "FAFAFA" "F4F4F5" "E4E4E7" "D4D4D8" "A1A1AA"
                "71717A" "52525B" "3F3F46" "27272A" "18181B"
            ],
            neutral: spectrum![
                "FAFAFA" "F5F5F5" "E5E5E5" "D4D4D4" "A3A3A3"
                "737373" "525252" "404040" "262626" "171717"
            ],
            stone: spectrum![
                "FAFAF9" "F5F5F4" "E7E5E4" "D6D3D1" "A8A29E"
                "78716C" "57534E" "44403C" "292524" "1C1917"
            ],
            red: spectrum![
                "FEF2F2" "FEE2E2" "FECACA" "FCA5A5" "F87171"
                "EF4444" "DC2626" "B91C1C" "991B1B" "7F1D1D"
            ],
            orange: spectrum![
                "FFF7ED" "FFEDD5" "FED7AA" "FDBA74" "FB923C"
                "F97316" "EA580C" "C2410C" "9A3412" "7C2D12"
            ],
            amber: spectrum![
                "FFFBEB" "FEF3C7" "FDE68A" "FCD34D" "FBBF24"
                "F59E0B" "D97706" "B45309" "92400E" "78350F"
            ],
            yellow: spectrum![
                "FEFCE8" "FEF9C3" "FEF08A" "FDE047" "FACC15"
                "EAB308" "CA8A04" "A16207" "854D0E" "713F12"
            ],
            lime: spectrum![
                "F7FEE7" "ECFCCB" "D9F99D" "BEF264" "A3E635"
                "84CC16" "65A30D" "4D7C0F" "3F6212" "365314"
            ],
            green: spectrum![
                "F0FDF4" "DCFCE7" "BBF7D0" "86EFAC" "4ADE80"
                "22C55E" "16A34A" "15803D" "166534" "14532D"
            ],
            emerald: spectrum![
                "ECFDF5" "D1FAE5" "A7F3D0" "6EE7B7" "34D399"
                "10B981" "059669" "047857" "065F46" "064E3B"
            ],
            teal: spectrum![
                "F0FDFA" "CCFBF1" "99F6E4" "5EEAD4" "2DD4BF"
                "14B8A6" "0D9488" "0F766E" "115E59" "134E4A"
            ],
            cyan: spectrum![
                "ECFEFF" "CFFAFE" "A5F3FC" "67E8F9" "22D3EE"
                "06B6D4" "0891B2" "0E7490" "155E75" "164E63"
            ],
            sky: spectrum![
                "F0F9FF" "E0F2FE" "BAE6FD" "7DD3FC" "38BDF8"
                "0EA5E9" "0284C7" "0369A1" "075985" "0C4A6E"
            ],
            blue: spectrum![
                "EFF6FF" "DBEAFE" "BFDBFE" "93C5FD" "60A5FA"
                "3B82F6" "2563EB" "1D4ED8" "1E40AF" "1E3A8A"
            ],
            indigo: spectrum![
                "EEF2FF" "E0E7FF" "C7D2FE" "A5B4FC" "818CF8"
                "6366F1" "4F46E5" "4338CA" "3730A3" "312E81"
            ],
            violet: spectrum![
                "F5F3FF" "EDE9FE" "DDD6FE" "C4B5FD" "A78BFA"
                "8B5CF6" "7C3AED" "6D28D9" "5B21B6" "4C1D95"
            ],
            purple: spectrum![
                "FAF5FF" "F3E8FF" "E9D5FF" "D8B4FE" "C084FC"
                "A855F7" "9333EA" "7E22CE" "6B21A8" "581C87"
            ],
            fuchsia: spectrum![
                "FDF4FF" "FAE8FF" "F5D0FE" "F0ABFC" "E879F9"
                "D946EF" "C026D3" "A21CAF" "86198F" "701A75"
            ],
            pink: spectrum![
                "FDF2F8" "FCE7F3" "FBCFE8" "F9A8D4" "F472B6"
                "EC4899" "DB2777" "BE185D" "9D174D" "831843"
            ],
            rose: spectrum![
                "FFF1F2" "FFE4E6" "FECDD3" "FDA4AF" "FB7185"
                "F43F5E" "E11D48" "BE123C" "9F1239" "881337"
            ],
        }
    }
}

pub fn hex_to_rgb(hex: &str) -> [u8; 3] {
    [
        u8::from_str_radix(&hex[0..2], 16).unwrap(),
        u8::from_str_radix(&hex[2..4], 16).unwrap(),
        u8::from_str_radix(&hex[4..6], 16).unwrap(),
    ]
}

pub fn rgb_to_color(rgb: [u8; 3]) -> Color {
    Color::RGB(rgb[0], rgb[1], rgb[2])
}
