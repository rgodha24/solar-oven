// This file has been generated by Specta. DO NOT EDIT.

export type ReflectorType = "Rectangular" | "Trapezoidal"

export type ReflectiveMaterial = "AF" | "TF" | "MS" | "RT" | "SR2000" | "SRV"

export type Oven = { abs: Absorber; window: WindowMaterial; inner_body: BodyMaterial; outer_body: BodyMaterial; insulator: Insulator; reflector_type: ReflectorType; reflective_material: ReflectiveMaterial; reflector_number: number }

export type Absorber = "BCS" | "TSC"

export type WindowMaterial = "SingleMylar" | "DoubleMylar"

export type Insulator = "N" | "FG" | "SF" | "DF" | "FG30"

export type BodyMaterial = "C" | "W16"
