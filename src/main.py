from typing import Final
import os
from dotenv import load_dotenv
import discord
from discord import Intents, app_commands
from discord.ext import commands
import server

# Loading some static stuff
load_dotenv("../config/secrets.env")
TOKEN: Final[str] = os.getenv("DISCORD_TOKEN")

# Bot setup
intents: Intents = Intents.default()
intents.message_content = True
# bot: Client = Client(intents=intents)
bot = commands.Bot(command_prefix="!", intents=intents)

# Commands
# [..]

# Bot startup
@bot.event
async def on_ready() -> None:
    print(f"{bot.user} is now running!")
    try:
        synced = await bot.tree.sync()
        print(f"Synced {len(synced)} command(s)")
    except Exception as e:
        print(e)

# Main entry point
if __name__ == "__main__":
    bot.run(token=TOKEN)
