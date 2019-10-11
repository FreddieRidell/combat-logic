import * as R from "ramda";

function randomString(length) {
	return Math.round(
		Math.pow(36, length + 1) - Math.random() * Math.pow(36, length),
	)
		.toString(36)
		.slice(1);
}

export type Polarity = "ALLY" | "FOE" | null;
type Query = {
	polarity: Polarity;
};

export type GambitId = string;
export type Gambit = {
	id: GambitId;
	query: Query;
	action: null;
};

export type CharacterId = string;
export type Character = {
	id: CharacterId;
	name: string;
	gambits: Record<GambitId, Gambit>;
};

type GambitsStore = Record<CharacterId, Character>;

type Action =
	| { type: "ADD_GAMBIT"; id: CharacterId }
	| { type: "DELETE_CHARACTER"; id: CharacterId }
	| { type: "NEW_CHARACTER" }
	| { type: "SET_NAME"; id: CharacterId; name: string }
	| {
			type: "SET_POLARITY";
			characterId: CharacterId;
			id: GambitId;
			value: Polarity;
	  };

export type Dispatch = (action: Action) => void;

function gambitsReducer(state: GambitsStore, action: Action): GambitsStore {
	console.log({ action });
	switch (action.type) {
		case "SET_POLARITY":
			return R.assocPath(
				[action.characterId, "gambits", action.id, "query", "polarity"],
				action.value,
				state,
			);

		case "ADD_GAMBIT":
			let id = randomString(16);

			const newGambit: Gambit = {
				id,
				query: {
					polarity: null,
				},
				action: null,
			};

			return R.assocPath([action.id, "gambits", id], newGambit, state);

		case "NEW_CHARACTER": {
			let id = randomString(16);

			return {
				...state,
				[id]: {
					id,
					name: "",
					gambits: {},
				},
			};
		}

		case "SET_NAME":
			return {
				...state,
				[action.id]: {
					...state[action.id],
					name: action.name,
				},
			};

		case "DELETE_CHARACTER":
			return R.omit([action.id], state);

		default:
			return state;
	}
}

export default function(state: GambitsStore, action: Action): GambitsStore {
	const output = gambitsReducer(state, action);

	localStorage.gambit = JSON.stringify(output);

	return output;
}
