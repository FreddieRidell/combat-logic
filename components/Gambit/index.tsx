import * as React from "react";

import { Polarity, Dispatch, Gambit, CharacterId } from "../../store";

const GambitComponent: React.FC<{
	characterId: CharacterId;
	gambit: Gambit;
	dispatch: Dispatch;
}> = ({
	characterId,
	gambit: {
		id,
		query: { polarity },
		action,
	},
	dispatch,
}) => {
	return (
		<div>
			<select
				value={polarity}
				onChange={(e: any) => {
					const value: Polarity = e.target.value;
					dispatch({
						type: "SET_POLARITY",
						characterId,
						id,
						value,
					});
				}}
			>
				<option value={null} />
				<option value="ALLY">Ally</option>
				<option value="FOE">Foe</option>
			</select>
		</div>
	);
};

export default GambitComponent;
