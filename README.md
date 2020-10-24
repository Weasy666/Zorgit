# Zorgit - A slightly rusty Git
[![Pipeline Status]()]()
[![Build Status]()]()
[![Coverage]()]()

## About the name
Initially I wanted to name it Benders, after...you know...[Bender](https://en.wikipedia.org/wiki/Bender_(Futurama)) from [Futurama](https://en.wikipedia.org/wiki/Futurama), because I wanted to make my own git platform, like [GitHub](https://github.com/) or [Gitea](https://gitea.io/), but with Black Jack and hoo`{peep}`rs. But then I thought...maybe that's not really professional and instead went with [Zorgit](https://www.mindat.org/min-29484.html) a plain old mineral. I know it is written Zorgite in English but I wanted to emphasize that this is a [Git](https://git-scm.com/) project, so I went with the German spelling to get rid of the annoying "e" at the end.
I also thought about naming it Rogs[rɒks] - *A Git that just rocks* or *[Rust](https://www.rust-lang.org/) open git service/solution/self-hosting* - but that would look like it was stolen from [Gogs](https://gogs.io) and there is no git visible in the name.

## Project
This project is my first "big" [Rust](https://www.rust-lang.org/) and web project, so...please bear with me and be nice to me 😜...I am sure there is a lot that could be done better or was even done wrong by me. I am happy about every constructive feedback I can get and I am also happy about feature and pull requests.

Because the name is a mineral/crystal, I would also like to have a logo that emphasizes that, so something like a crystal dendrite or collection of crystals that also somehow remind one of a sourcetree. And I also would like to have a mascot, Rocky, that should be a stone, mineral, crystal golem/creature. So if someone is good at that stuff, feel free to say hello in the corresponding issue #1.

The current goal is to make this thing usable for users, with public and private repositories. The long term goal is to achieve feature parity with [Gitea](https://gitea.io/) and to integrate a CI/CD like [Drone](https://drone.io), but preferably one written in [Rust](https://www.rust-lang.org/) (maybe something like [cargo-make](https://github.com/sagiegurari/cargo-make), [Toast](https://github.com/stepchowfun/toast), [Lorikeet](https://github.com/cetra3/lorikeet), [Cargo-Wharf](https://github.com/denzp/cargo-wharf) for container first builds, etc. but haven't evaluated them yet) or at least provide some kind of interface to integrate external solutions with webhooks.

### Current status
Currently the project is in a very early state and should not, I repeat **NOT**, be used in production. If you do, you do it at your own risk.

- [x] Basic user accounts and authentication
- [x] Project/Repository ownership for single users (Organizations are not implemented yet)
- [x] Project/Repository visibility and access restriction with private repositories
- [x] Project can be created and optionally initialized with a new bare repository in the configured Zorgit directory
- [x] Issues can be created with assignees and labels and comments can be added. Something like user mentions or parsed and linked commits are not implemented yet.
- [x] Repository entries can be navigated and single files can be opened and displayed
- [x] Last commit is shown for each entry and the newest one above in the table header
- [x] Commits and branches can be viewed
- [x] Dashboard with all projects of a user

So as you can see, the most basic stuff is working, but a lot is still missing. It is not possible to delete or archive a project and currently all settings pages (user and project) don't exist. Also general access to projects is limited, but there is still no difference between a collaborator or the project owner and even worse, public projects allow every logged in user to edit labels, commits and so on.
I would also like to restructure the source code, but have still not decided on what the best solution is. CSS also needs some restructuring and extracting colors to CSS values to allow theming. And the authentication system also needs a complete overhaul and the database operations too.

Also...to my shame...there are currently no unit tests 😅

## Security
I tried to stick to this [Guide](https://stackoverflow.com/questions/549/the-definitive-guide-to-form-based-website-authentication) about *Form based website authentication* on Stackoverflow for the login and password security.
No CORS or such is currently implemented because I have no prior knowledge of that and still have to read it up.

> [![](assets/password_strength.png)](https://xkcd.com/936/)
> Don't think to much about it, this image is just here to test the image functionality.
