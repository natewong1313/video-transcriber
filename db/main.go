package main

import (
	"context"
	"log"
	"os"

	_ "github.com/jackc/pgx/v5/stdlib"
	"github.com/joho/godotenv"
	"github.com/pressly/goose/v3"
)

const MIGRATIONS_DIR = "./migrations"

// based on https://learning-cloud-native-go.github.io/docs/database-and-migrations/
func main() {
	args := os.Args[1:]
	if len(args) < 1 {
		log.Fatal("missing command argument")
	}
	cmd := args[0]

	err := godotenv.Load("../.env")
	if err != nil {
		log.Fatal("could not find .env file")
	}

	dbURL := os.Getenv("DATABASE_URL")
	if dbURL == "" {
		log.Fatal("DATABASE_URL not loaded")
	}

	os.MkdirAll(MIGRATIONS_DIR, os.ModePerm)

	db, err := goose.OpenDBWithDriver("pgx", dbURL)
	if err != nil {
		log.Fatalf("could not open db: %v", err)
	}
	defer db.Close()

	if err := goose.RunContext(context.Background(), cmd, db, MIGRATIONS_DIR, args[1:]...); err != nil {
		log.Fatalf("failed to migrate: %v", err)
	}
}
