CREATE TYPE gameresult AS ENUM (
    'win',
    'loss',
    'tie'
);

CREATE TABLE game (
	game_id VARCHAR NOT NULL,
	started_at TIMESTAMP NOT NULL DEFAULT NOW(),
	ended_at TIMESTAMP NULL,
	"result" gameresult NULL,
	CONSTRAINT game_pk PRIMARY KEY (game_id)
);
