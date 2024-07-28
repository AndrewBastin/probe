<p align="center">
  <h1 align="center">probe</h1>
</p>
<p align="center">
  <i>Empowering developers to support the open source ecosystem.</i>
</p>

## Manifesto
"Open source is not about free software, it's about free people."
— Richard Stallman

## What is probe?
probe is a web app that scans your package.json file against NPM and provides detailed information about the open source libraries you're using. It aims to increase awareness and support for the open source ecosystem by making it easier for developers to contribute back to the projects they rely on.
We developed probe to address the challenge of sustaining open source projects and encouraging more developers to contribute, either through code or financial support.
An insightful tool that inspires developers to give back to the community and fosters a more sustainable open source ecosystem.

[Screenshot placeholder]

## Inspiration
Project probe is inspired by the ongoing discussions about open source sustainability and the need for better ways to support maintainers.

#### We believe supporting open source is everyone's responsibility.
The health of the open source ecosystem affects everyone who uses software. By making it easier to identify and support the projects we depend on, we can ensure a more sustainable future for open source.

## How?

probe scans your package.json file and queries the NPM registry for each dependency.
It retrieves and presents key information about each package, including funding options, recent commit activity, and contribution guidelines.
Users can easily see which projects might need support and how they can help, whether through code contributions or financial backing through the given filters and options to surface key metrics related to sustainability of a project.

While there are tools that scan for vulnerabilities or outdated packages, probe focuses on the human side of open source - the maintainers and contributors who make these projects possible.

## Features
 - **Dependency scanning**: Automatically scan your package.json file and retrieve information from NPM.
 - **Funding information**: See which of your dependencies accept donations and how you can contribute.
 - **Commit activity**: See stats on recent commit activities. Currently only implemented for Github Repositories. [Partially implemented]
 - **Issue Response Health**: See how much time it takes for issues to get resolved.

## Planned Features
 - **Integration with other package ecosystems**: Integrate with Cargo, PyPi, pkgs.go.dev and more.
 - **Analytics on PR reviews and authors**: Insights into cycle time and general PR related statistics.

## Demo
[Web app demo placeholder]

## Usage
You can use the hosted instance of the app at [https://dev-probe.app](https://dev-probe.app).

## Built with
 - JavaScript: Vue, TailwindCSS
 - Rust: Rocket
 - Services: Together AI, Jina Reader API (for parsing funding information from funding websites)

## Development
- If you have docker and docker compose installed, you can run `docker compose up dev` to run the dev mode container.
- You need the following environment variables in a `.env` file on the root of the repo before starting the dev container
    - `GITHUB_TOKEN`: A Github API Token
    - `TOGETHER_AI_API_KEY`: Together AI API Key (used for funding page data scraping)
    - `JINA_READER_API_KEY`: Jina Reader API Key (user for funding page data scraping)

# Authors
- [Andrew Bastin](https://github.com/AndrewBastin)
- [Riya Mathew](https://github.com/RiyaMathew-11)

# License
This project is licensed under the MIT License - see the LICENSE file for details.

<br />
<br />
<br />

<p align="center">
  <b>Empower developers to support the open source ecosystem.<br>Because we believe <ins>every contribution matters.</ins></b>
</p>
