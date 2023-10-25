CREATE TABLE round (
	game_id VARCHAR NOT NULL,
	round_id SERIAL NOT NULL,
    opponent_move BOOLEAN NOT NULL,
    move_x SMALLINT NULL,
    move_y SMALLINT NULL,
	CONSTRAINT round_pk PRIMARY KEY (game_id, round_id),
    CONSTRAINT round_game_fk FOREIGN KEY (game_id) REFERENCES game (game_id)
);
