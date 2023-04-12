# git-check-cli

cli to help check and update git repos

## Installation

Create a config file in

## Use

The path in projects list always needs to be absolute, relative paths will not work correctly

## Uninstall

```sh

```

## Future Ideas


Display idea

- Think about default, maybe verbose is better and the other as an short option

```sh
git-check check
```

```
project-name1 [ commits: n ] [ origin: to-push: n ]
project-name2 [ up to date ]
project-name3 [ commits: n ] [ origin1: to-push: n, to-pull: n ] [ origin2: to-pull: n ]

```

---

```sh
git-check check --verbose
```

```
project-name1 - PROJECT/NAME/PATH/1
└──┬ Local changes:
   │    └─── ?? src/main.rs
   ├ origin changes:
   │    └──┬[To push: 3]
   │       │    └──┬ ab12cd3 add all
   │       │       ├ ef45gh6 add all
   │       │       └ ij78kl9 add all
   │       └[To pull: 1]
   │            └─── ab12cd3 add all
   └ origin2 changes:
        └───[To pull: 2]
                └──┬ ef45gh6 add all
                   └ ij78kl9 add all
project-name2 - PROJECT/NAME/PATH/2
└──┬ Local changes: OK
   └ origin changes: Ok
project-name3 - PROJECT/NAME/PATH/3
└──┬ Local changes: OK
   └ No Remotes
```

