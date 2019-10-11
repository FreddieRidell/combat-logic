import * as React from "react";

import gambitsReducer from "../store";

import Player from "../components/Player";

import "./main.css";

const Home = () => {
	const [store, dispatch] = React.useReducer(
		gambitsReducer,
		process.browser ? JSON.parse(localStorage.gambit || "{}") : {},
	);

	return (
		<form
			onSubmit={e => {
				e.preventDefault();
			}}
		>
			<button
				onClick={() =>
					dispatch({
						type: "NEW_CHARACTER",
					})
				}
			>
				New Character
			</button>
			{Object.keys(store).map(playerKey => (
				<Player
					key={playerKey}
					player={store[playerKey]}
					dispatch={dispatch}
				/>
			))}

			<output>{JSON.stringify(store, null, 2)}</output>
		</form>
	);
};

export default Home;
