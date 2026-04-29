# JEEDZ-Bot
PHOLX Internship Project : Rust Based Discord Assistant Bot for Making an appointment to Google Calendar, Simple Monitoring Network and Server
<<<<<<< HEAD
=======


# 🤖 JEEDZ Bot

A Discord bot built in Rust. Jeedz Bot acts as a team productivity and DevOps assistant, featuring seamless Google Calendar scheduling, a background Network Uptime Monitor, and a Mini Security Operations Center (SOC) log analyzer.

## ✨ Features

* **🗓️ Smart Meeting Scheduler (`/meeting`):** * Schedule team meetings directly from Discord via slash commands.
  * Automatically creates events on a shared Google Calendar.
  * Supports custom durations, specific Discord role tagging, and dynamic timezones (defaults to Asia/Bangkok).
  * Returns a professional Rich Embed with a clickable Google Calendar link.
* **🛡️ Mini SOC (Log Monitor):** * Runs as an asynchronous background task.
  * Continuously tails server log files in real-time.
  * Uses regex threat signatures to detect and alert the team of Brute Force SSH attempts or SQL Injection payloads.
* **🌐 Network Uptime Monitor:** * Periodically checks the health of critical APIs or web servers.
  * Acts as a "silent guardian"—only alerts the Discord channel when a server goes offline or recovers.
* **👋 Server Greetings:** * Automatically welcomes new users and says goodbye when members leave.

## 🛠️ Tech Stack

* **Language:** `Rust`
* **Discord API:** `serenity` & `poise` framework
* **Async Runtime:** `tokio`
* **Google Integration:** `google-calendar3`, `yup-oauth2`
* **Networking & TLS:** `reqwest`, `hyper`, `rustls` (Ring crypto provider)
* **Log Tailing:** `linemux`

## 📋 Prerequisites

Before running this bot, you will need:
1. [Rust & Cargo](https://rustup.rs/) installed.
2. A Discord Bot Token (from the [Discord Developer Portal](https://discord.com/developers/applications)).
3. **IMPORTANT:** The **"Server Members Intent"** must be enabled in your Discord Developer Portal for the greetings feature to work.
4. A Google Cloud Service Account with Google Calendar API enabled.

## 🚀 Installation & Setup

### 1. Clone the Repository
```bash
git clone [https://github.com/yourusername/jeedz-bot.git](https://github.com/yourusername/jeedz-bot.git)
cd jeedz-bot
>>>>>>> d5398a6 (Complete JEEDZ Bot v0.0.1)
