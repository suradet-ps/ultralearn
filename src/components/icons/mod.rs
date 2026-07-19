use leptos::prelude::*;

/// Generic inline-SVG icon component. Mirrors the Lucide icons used in the Vue
/// app. Inner SVG markup is passed as `children` (paths/circles/etc.).
#[component]
pub fn Icon(
    /// SVG inner markup (paths/circles/etc.) for a 24x24 viewBox.
    children: Children,
    /// Pixel size (renders as width/height).
    #[prop(default = 24)]
    size: u32,
    /// Stroke color override; defaults to `currentColor`.
    color: Option<String>,
    /// Extra classes for the <svg>.
    class: Option<String>,
    /// Aria label / title for accessibility.
    label: Option<String>,
) -> impl IntoView {
    let color = color.unwrap_or_else(|| "currentColor".to_string());
    let title = label.map(|l| view! { <title>{l}</title> });
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width=size
            height=size
            viewBox="0 0 24 24"
            fill="none"
            stroke=color.clone()
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class=class
            role="img"
        >
            {title}
            {children()}
        </svg>
    }
}

macro_rules! icon {
    ($name:ident, $paths:expr) => {
        #[component]
        pub fn $name(
            #[prop(default = 24)] size: u32,
            #[prop(optional)] color: Option<String>,
            #[prop(optional)] class: Option<String>,
            #[prop(optional)] label: Option<String>,
        ) -> impl IntoView {
            view! {
                <Icon size=size color=color class=class label=label>
                    {$paths}
                </Icon>
            }
        }
    };
}

icon!(
    Sun,
    view! {
        <>
            <circle cx="12" cy="12" r="4" />
            <path d="M12 2v2" />
            <path d="M12 20v2" />
            <path d="m4.93 4.93 1.41 1.41" />
            <path d="m17.66 17.66 1.41 1.41" />
            <path d="M2 12h2" />
            <path d="M20 12h2" />
            <path d="m6.34 17.66-1.41 1.41" />
            <path d="m19.07 4.93-1.41 1.41" />
        </>
    }
);

icon!(
    Moon,
    view! { <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" /> }
);

icon!(
    BookOpen,
    view! {
        <>
            <path d="M12 7v14" />
            <path d="M3 18a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h5a4 4 0 0 1 4 4 4 4 0 0 1 4-4h5a1 1 0 0 1 1 1v13a1 1 0 0 1-1 1h-6a3 3 0 0 0-3 3 3 3 0 0 0-3-3z" />
        </>
    }
);

icon!(
    ArrowRight,
    view! {
        <>
            <path d="M5 12h14" />
            <path d="m12 5 7 7-7 7" />
        </>
    }
);

icon!(
    ArrowLeft,
    view! {
        <>
            <path d="m12 19-7-7 7-7" />
            <path d="M19 12H5" />
        </>
    }
);

icon!(
    Sparkles,
    view! {
        <>
            <path d="M9.937 15.5A2 2 0 0 0 8.5 14.063l-6.135-1.582a.5.5 0 0 1 0-.962L8.5 9.936A2 2 0 0 0 9.937 8.5l1.582-6.135a.5.5 0 0 1 .963 0L14.063 8.5A2 2 0 0 0 15.5 9.937l6.135 1.581a.5.5 0 0 1 0 .964L15.5 14.063a2 2 0 0 0-1.437 1.437l-1.582 6.135a.5.5 0 0 1-.963 0z" />
            <path d="M20 3v4" />
            <path d="M22 5h-4" />
            <path d="M4 17v2" />
            <path d="M5 18H3" />
        </>
    }
);

icon!(
    Download,
    view! {
        <>
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
            <polyline points="7 10 12 15 17 10" />
            <line x1="12" y1="15" x2="12" y2="3" />
        </>
    }
);

icon!(
    Trash2,
    view! {
        <>
            <path d="M3 6h18" />
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" />
            <line x1="10" y1="11" x2="10" y2="17" />
            <line x1="14" y1="11" x2="14" y2="17" />
        </>
    }
);

icon!(
    CheckCircle2,
    view! {
        <>
            <circle cx="12" cy="12" r="10" />
            <path d="m9 12 2 2 4-4" />
        </>
    }
);

icon!(Circle, view! { <circle cx="12" cy="12" r="10" /> });

icon!(
    Plus,
    view! {
        <>
            <path d="M5 12h14" />
            <path d="M12 5v14" />
        </>
    }
);

icon!(
    X,
    view! {
        <>
            <path d="M18 6 6 18" />
            <path d="m6 6 12 12" />
        </>
    }
);

icon!(Check, view! { <path d="M20 6 9 17l-5-5" /> });

icon!(ChevronLeft, view! { <path d="m15 18-6-6 6-6" /> });

icon!(ChevronRight, view! { <path d="m9 18 6-6-6-6" /> });

icon!(
    RotateCcw,
    view! {
        <>
            <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
            <path d="M3 3v5h5" />
        </>
    }
);

icon!(Play, view! { <polygon points="6 3 20 12 6 21 6 3" /> });

icon!(
    Pause,
    view! {
        <>
            <rect x="14" y="4" width="4" height="16" rx="1" />
            <rect x="6" y="4" width="4" height="16" rx="1" />
        </>
    }
);

icon!(
    PenLine,
    view! {
        <>
            <path d="M12 20h9" />
            <path d="M16.376 3.622a1 1 0 0 1 3.002 3.002L7.368 18.635a2 2 0 0 1-.855.506l-2.872.838a.5.5 0 0 1-.62-.62l.838-2.872a2 2 0 0 1 .506-.854z" />
        </>
    }
);

icon!(
    AlertTriangle,
    view! {
        <>
            <path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" />
            <line x1="12" y1="9" x2="12" y2="13" />
            <line x1="12" y1="17" x2="12.01" y2="17" />
        </>
    }
);

icon!(
    FlaskConical,
    view! {
        <>
            <path d="M10 2v7.31" />
            <path d="M14 9.3V2" />
            <path d="M8.5 2h7" />
            <path d="M14 9.3a6.5 6.5 0 1 1-4 0" />
            <path d="M5.58 16.5h12.85" />
        </>
    }
);

icon!(
    Calendar,
    view! {
        <>
            <path d="M8 2v4" />
            <path d="M16 2v4" />
            <rect width="18" height="18" x="3" y="4" rx="2" />
            <path d="M3 10h18" />
        </>
    }
);
