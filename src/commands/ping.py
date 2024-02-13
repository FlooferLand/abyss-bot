import discord
from main import bot

@bot.tree.command(name="ping", description="Checks the bot's status")
async def ping(interaction: discord.Interaction):
    await interaction.response.send_message(f":eye: _ :eye:\n_(I see all is well!)_", ephemeral=True)
