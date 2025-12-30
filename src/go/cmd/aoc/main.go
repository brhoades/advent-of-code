package main

import (
	"context"
	"fmt"
	"log"
	"os"

	"brod.es/aoc/internal/twentyfive/one"
	"brod.es/aoc/internal/twentyfive/two"
	"github.com/urfave/cli/v3"
)

func main() {
	var year, day, part int
	var extra string
	cmd := &cli.Command{
		UseShortOptionHandling: true,
		Commands: []*cli.Command{
			{
				Name:  "run",
				Usage: "Run a specific year day and part of AoC",
				Arguments: []cli.Argument{
					&cli.IntArg{Name: "year", Destination: &year},
					&cli.IntArg{Name: "day", Destination: &day},
					&cli.IntArg{Name: "part", Destination: &part},
					&cli.StringArg{Name: "extra", Destination: &extra},
				},
				Action: func(ctx context.Context, cmd *cli.Command) error {
					if year != 25 {
						return fmt.Errorf("unknown year: %d", year)
					}

					switch day {
					case 1:
						return one.One(part, extra)
					case 2:
						return two.Main(part, extra)
					}

					return fmt.Errorf("unknown day: %d", day)
				},
			},
		},
	}

	if err := cmd.Run(context.Background(), os.Args); err != nil {
		log.Fatal(err)
	}
}
