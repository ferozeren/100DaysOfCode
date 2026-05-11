import asyncio
import os

import discord
import requests
import yt_dlp
from discord.ext import commands
from dotenv import load_dotenv

load_dotenv()
token = os.getenv("BOT_TOKEN")


def get_spotify_info(spotify_url: str) -> None | dict:
    """Get Title, Artist, and YouTube link from the Spotify Link, return None if failed"""
    api_url = f"https://api.song.link/v1-alpha.1/links?url={spotify_url}"

    try:
        response = requests.get(api_url)
        response.raise_for_status()
        data = response.json()

        # Get the Unique ID for the track metadata
        entity_id = data.get("entityUniqueId")
        metadata = data.get("entitiesByUniqueId", {}).get(entity_id, {})

        title = metadata.get("title", "Unknown Title")
        artist = metadata.get("artistName", "Unknown Artist")

        # Get the YouTube URL
        links = data.get("linksByPlatform", {})
        yt_url = links.get("youtube", {}).get("url") or links.get(
            "youtubeMusic", {}
        ).get("url")

        return {"title": title, "artist": artist, "yt_url": yt_url}

    except Exception as e:
        print(f"Error: {e}")
        return None


# Intents
intents = discord.Intents.default()
intents.message_content = True
intents.voice_states = True

bot = commands.Bot(command_prefix="?", intents=intents)

ytdl_format_options = {
    "format": "bestaudio/best",
    "noplaylist": True,
    "quiet": True,
}


ffmpeg_options = {
    "before_options": "-reconnect 1 -reconnect_streamed 1 -reconnect_delay_max 5",
    "options": "-vn",  # ignore video, process audio only
}


ytdl = yt_dlp.YoutubeDL(ytdl_format_options)


@bot.event
async def on_ready():
    print(f"Logged in as {bot.user}!")


@bot.command(name="join", aliases=["jn", "connect", "cnt", "cn"])
async def join_vc(ctx):
    if ctx.author.voice is None:
        return await ctx.send("You need to be in a voice channel for me to join!")

    voice_channel = ctx.author.voice.channel

    if ctx.voice_client is not None:
        return await ctx.voice_client.move_to(voice_channel)

    await voice_channel.connect()
    await ctx.send(f"Successfully joined **{voice_channel.name}**!")


@bot.command(name="leave")
async def leave_vc(ctx):
    if ctx.voice_client is not None:
        await ctx.voice_client.disconnect()
        await ctx.send("Disconnected from the voice channel.")
    else:
        await ctx.send("I'm not in a voice channel right now.")


@bot.command(name="play", aliases=["pl", "start"])
async def play(ctx, *, query: str):
    """Plays audio from a YouTube URL or search query."""

    # Ensure bot is in a voice channel first
    if ctx.voice_client is None:
        if ctx.author.voice:
            await ctx.author.voice.channel.connect()
        else:
            return await ctx.send("You are not connected to a voice channel.")

    # Stop current audio if something is already playing
    if ctx.voice_client.is_playing():
        ctx.voice_client.stop()

    await ctx.send(f"Searching for: `{query}`...")

    # Extract audio info using yt-dlp
    # Running in a separate thread so it doesn't block the bot
    loop = asyncio.get_event_loop()
    try:
        # If it's not a URL, search using yt-dlp with "ytsearch: "
        if not query.startswith("http"):
            query = f"ytsearch: {query}"
        if query.startswith("https://open.spotify.com"):
            if info := get_spotify_info(query):
                if not info["yt_url"]:
                    query = f"ytsearch: {info['title']} {info['artist']}"
                else:
                    query = info["yt_url"]

        data = await loop.run_in_executor(
            None, lambda: ytdl.extract_info(query, download=False)
        )

        if "entries" in data:
            data = data["entries"][0]  # First search result

        url = data["url"]
        title = data.get("title", "Unknown Title")

        # FFmpeg audio source and play it on discord.
        source = discord.FFmpegPCMAudio(url, **ffmpeg_options)
        ctx.voice_client.play(
            source, after=lambda e: print(f"Player error: {e}") if e else None
        )

        await ctx.send(f"Now playing: **{title}**")

    except Exception as e:
        await ctx.send("An error occurred while trying to play the audio.")
        print(f"Error: {e}")


@bot.command(name="stop", aliases=["st", "quit", "qt"])
async def stop(ctx):
    """Stops the audio"""

    if ctx.voice_client and ctx.voice_client.is_playing():
        ctx.voice_client.stop()
        await ctx.send("Audio stopped.")
    else:
        await ctx.send("Nothing is playing right now.")


@bot.command(name="pause", aliases=["ps", "hold"])
async def pause(ctx):
    """Pause the audio"""

    if ctx.voice_client and ctx.voice_client.is_playing():
        ctx.voice_client.pause()
        await ctx.send("⏸️ **Audio paused.**")
    else:
        await ctx.send("Nothing is playing right now.")


@bot.command(name="resume", aliases=["unpause", "rs", "re"])
async def resume(ctx):
    """Resume the audio"""

    if ctx.voice_client and ctx.voice_client.is_paused():
        ctx.voice_client.resume()
        await ctx.send("▶️ **Audio resumed.**")
    else:
        await ctx.send("The audio isn't paused, or I'm not in a voice channel!")


if token:
    bot.run(token)
else:
    print("Token not found, exiting...")
