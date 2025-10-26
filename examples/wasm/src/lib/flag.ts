import { PresetFlag, type CustomFlag } from "pride-overlay";

export const CATPPUCCIN: CustomFlag = {
    name: "Catppuccin",
    colours: ["#ED8796", "#F5A97F", "#F5A97F", "#EED49F", "#A6DA95", "#7DC4E4", "#C6A0F6"],
};

enum CustomFlags {
    Catppuccin
}

export const AllFlag = {
    ...PresetFlag,
    ...CustomFlags
} as const;
export type AllFlag = PresetFlag | CustomFlags;