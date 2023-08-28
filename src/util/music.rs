use poise::serenity_prelude::{Color, CreateEmbed, User};
use songbird::input::{Input, Metadata};

// Default embed for music
pub fn send_music_embed(metadata: Box<Metadata>, embed: &mut CreateEmbed, author: &User) {
    let duration = format_duration(metadata.duration.unwrap().as_secs());
    if let Metadata {
        title: Some(title),
        thumbnail: Some(thumbnail),
        source_url: Some(source_url),
        artist: Some(artist),
        date: Some(date),
        ..
    } = *metadata
    {
        embed
            .title(title)
            .thumbnail(thumbnail)
            .author(|f| {
                f.icon_url(author.avatar_url().unwrap())
                    .name(author.name.clone())
            })
            .color(Color::from_rgb(0, 128, 128))
            .url(source_url)
            .fields(vec![
                ("author", artist, true),
                ("duration", duration, true),
                (
                    "upload date",
                    format!("{}-{}-{}", &date[4..6], &date[6..], &date[..4]),
                    true,
                ),
            ]);
    }
}

// Format duration to represent hours, minutes, seconds
pub fn format_duration(duration: u64) -> String {
    let seconds = duration % 60;
    let minutes = (duration / 60) % 60;
    let hours = (duration / 60) / 60;

    if hours != 0 {
        format!("{:0>2}:{:0>2}:{:0>2} hours", hours, minutes, seconds)
    } else if minutes != 0 {
        format!("{:0>2}:{:0>2} minutes", minutes, seconds)
    } else {
        format!("{} seconds", seconds)
    }
}

// Search song and returns data stream
pub async fn search(search_query: String) -> Result<Input, String> {
    if search_query.starts_with("https://www.youtube.com/") {
        songbird::input::ytdl(search_query)
            .await
            .map_err(|e| format!("ytdl failed {:#?}", e))
    } else if search_query.contains("https://www.youtube.com/playlist?list") {
        info!("youtube playlist not yet implemented");
        songbird::input::ytdl(search_query)
            .await
            .map_err(|e| format!("ytdl failed {:#?}", e))
    } else if search_query.contains("https://open.spotify.com/") {
        info!("spotify not yet implemented");
        songbird::input::ytdl(search_query)
            .await
            .map_err(|e| format!("ytdl failed {:#?}", e))
    } else {
        songbird::input::ytdl_search(search_query)
            .await
            .map_err(|e| format!("ytdl_search failed {:#?}", e))
    }
}

pub async fn youtube_playlist(search_query: String) {
    let version = "yt-dlp";
}
