import * as React from "react";

import Gambit from "../Gambit";
import { Dispatch, Character } from "../../store";

const Player: React.FC<{
	player: Character;
	dispatch: Dispatch;
}> = ({ player: { id, name, gambits }, dispatch }) => {
	return (
		<fieldset>
			<legend>{name}</legend>
			<input
				value={name}
				onChange={e =>
					dispatch({ type: "SET_NAME", id, name: e.target.value })
				}
			/>
			<ol>
				{Object.values(gambits).map(gambit => (
					<Gambit
						dispatch={dispatch}
						gambit={gambit}
						key={gambit.id}
						characterId={id}
					/>
				))}
			</ol>
			<button onClick={() => dispatch({ type: "ADD_GAMBIT", id })}>
				New Gambit
			</button>
			<button onClick={() => dispatch({ type: "DELETE_CHARACTER", id })}>
				Delete
			</button>
		</fieldset>
	);
};

export default Player;
