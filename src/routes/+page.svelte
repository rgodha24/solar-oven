<script lang="ts">
	import { getData } from '$lib';
	import {
		Absorber,
		WindowMaterial,
		Insulator,
		BodyMaterial,
		ReflectorType,
		ReflectiveMaterial
	} from 'solar-oven';

	const data = getData({
		abs: 'BCS',
		window: 'DoubleMylar',
		insulator: 'FG30',
		inner_body: 'W16',
		outer_body: 'W16',
		reflector_type: 'Trapezoidal',
		reflector_number: 4,
		reflective_material: 'TF'
	});

	console.log(data);
</script>
