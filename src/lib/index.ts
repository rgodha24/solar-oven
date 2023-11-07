import { graph_data, GraphDataResponse, GraphData, oven_from_json } from 'solar-oven';
import type { Oven as JSONOven } from './types';

export function getData(o: JSONOven) {
	const oven = oven_from_json(JSON.stringify(o));
	if (!oven) {
		return [];
	}

	let responseType: GraphDataResponse = GraphDataResponse.Tio;
	let reflectorML = 3.0;

	const data = graph_data(oven, responseType, reflectorML);

	oven.free();

	return data.map((d) => {
		const x = d.h;
		const y = d.insulator_thickness;
		const z = d.z;

		d.free();
		return [x, y, z];
	});
}
