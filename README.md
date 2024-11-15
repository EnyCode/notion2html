# notion2html

get your notion pages and turn them into html!

this was made as a proof of concept for my blog (coming soon 👀)

## how do I use it?

- make sure you have [rust](https://rust-lang.org) and cargo installed
- install it! `cargo install notion2html`
- make a [notion integration](https://www.notion.so/profile/integrations) and get a secret - make sure you add the read content scope!
- find the page you want to convert, open the hamburger menu and under "Connect to" select your app
- get the page id from the url (eg. https://www.notion.so/enymc/Sailing-the-High-Seas-428f22ca736843a992aa699a67787288 should be 428f22ca736843a992aa699a67787288 as the id)
- run it! `NOTION_TOKEN=<your token here> notion2html <page id> > out.html`
- your html will now be in `out.html`!

Licensed under MPL 2.0
