# Broken Telegram Bot

[![Build Status](https://travis-ci.org/zoobestik/borken_bot.svg?branch=master)](https://travis-ci.org/zoobestik/borken_bot)
[![Docker Image](https://img.shields.io/badge/ready%20for-docker-ff69b4.svg?style=flat)](https://hub.docker.com/r/zoobestik/gamebot/)
[![Telegram](https://img.shields.io/badge/join%20to-telegram-blue.svg?style=flat)](https://telegram.me/borken_bot)

A simple telegram bot. More in [CHANGELOG](CHANGELOG.md)

## Usage

**Step 1:** Installing from command line[1]:

```bash
curl https://raw.githubusercontent.com/zoobestik/borken_bot/master/docker-compose.yml > docker-compose.yml
echo "BOT_TELEGRAM_TOKEN=<...token...>" >> .env
```

[1]: Placeholder should be replaced by:

* `<...token...>` – token to access for HTTP API;

More information in [Telegram Bot Api page](https://core.telegram.org/bots#3-how-do-i-create-a-bot).

**Step 2:** Start a bot server instance:

```bash
docker-compose up
```

## Development

```bash
git config core.hooksPath scripts/.githooks
```

## Acknowledgements

[![Develop By](https://img.shields.io/badge/develop%20by-zoobestik-blue.svg?style=flat)](https://ru.linkedin.com/in/kbchernenko) [![MIT license](https://img.shields.io/badge/license-MIT-brightgreen.svg)](http://opensource.org/licenses/MIT)
