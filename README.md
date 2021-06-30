# Ants Simulator
A simple ants simulator using [ggez](https://ggez.rs/) Rust library for GUI.
The ants will search for food, and when they find it they walk toward home and leaving signs so the others ants can find easily the food.

![ants-simulator](./ants.gif)

## The json file
The simulator will run base on information (about foods, home the ants location) in a json file:
```
{
	"Ants": [
		{
			"x": 550,
			"y": 350,
            "amount": 250
		}
	],
	"Home": [
		{
		"x": 550,
		"y": 350
		}
	],
	"Food": [
		{
			"x": 350,
			"y": 1200,
            "amount": 500
		},
		{
			"x": 700,
			"y": 1250,
            "amount": 500
		}
	]
}
```

You have to run the programe with a path to a json file. 

## How to run
```
cargo run --release ./info.json
```
Note that if you don't use the release flag, the GUI might be updating slow due to unoptimized compiled code.
