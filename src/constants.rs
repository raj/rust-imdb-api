/// API constants
pub const HOST: &str = "api.imdbws.com";
pub const BASE_URI: &str = "https://api.imdbws.com";
pub const SEARCH_BASE_URI: &str = "https://v2.sg.media-imdb.com";
pub const USER_AGENT: &str = "IMDb/8.3.1 (iPhone9,4; iOS 11.2.1)";
pub const APP_KEY: &str = "76a6cc20-6073-4290-8a2c-951b4580ae4a";

/// Simple GET endpoints that require IMDB ID substitution
pub const SIMPLE_GET_ENDPOINTS: &[(&str, &str)] = &[
    ("get_name_images", "/name/{imdb_id}/images"),
    ("get_name_videos", "/name/{imdb_id}/videos"),
    ("get_title_metacritic_reviews", "/title/{imdb_id}/metacritic"),
    ("get_title_user_reviews", "/title/{imdb_id}/userreviews"),
    ("get_title_videos", "/title/{imdb_id}/videos"),
    ("get_title_images", "/title/{imdb_id}/images"),
    ("get_title_companies", "/title/{imdb_id}/companies"),
    ("get_title_technical", "/title/{imdb_id}/technical"),
    ("get_title_trivia", "/title/{imdb_id}/trivia"),
    ("get_title_goofs", "/title/{imdb_id}/goofs"),
    ("get_title_soundtracks", "/title/{imdb_id}/soundtracks"),
    ("get_title_news", "/title/{imdb_id}/news"),
    ("get_title_plot", "/title/{imdb_id}/plot"),
    ("get_title_plot_synopsis", "/title/{imdb_id}/plotsynopsis"),
    ("get_title_plot_taglines", "/title/{imdb_id}/taglines"),
    ("get_title_versions", "/title/{imdb_id}/versions"),
    ("get_title_releases", "/title/{imdb_id}/releases"),
    ("get_title_quotes", "/title/{imdb_id}/quotes"),
    ("get_title_connections", "/title/{imdb_id}/connections"),
    ("get_title_genres", "/title/{imdb_id}/genres"),
    ("get_title_similarities", "/title/{imdb_id}/similarities"),
    ("get_title_awards", "/title/{imdb_id}/awards"),
    ("get_title_ratings", "/title/{imdb_id}/ratings"),
    ("get_title_credits", "/title/{imdb_id}/fullcredits"),
    ("get_name", "/name/{imdb_id}/fulldetails"),
    ("get_name_filmography", "/name/{imdb_id}/filmography"),
    ("get_title", "/title/{imdb_id}/auxiliary"),
];

/// Chart endpoints
pub const CHART_TITLEMETER: &str = "/chart/titlemeter";
pub const CHART_TVMETER: &str = "/chart/tvmeter";
pub const CHART_MOVIMETER: &str = "/chart/moviemeter";
