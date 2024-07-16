import { invoke } from "@tauri-apps/api";
import type { Optional } from "./utils";

export const getWindowInfo = async (rawHandle: number) => {
	const [_title, _className, _processName] = await Promise.all([
		invoke<string | null>("get_window_title", { rawHandle }),
		invoke<string | null>("get_window_class", { rawHandle }),
		invoke<string | null>("get_window_process_name", { rawHandle }),
	]);

	const title = _title ?? "";
	const className = _className ?? "";
	const processName = (_processName ?? "").split("\\").at(-1) ?? "";

	return {
		title,
		className,
		processName,
	};
};

export type RawWindowRule = {
	command: string;
	match_process_name?: string;
	match_class_name?: string;
	match_title?: string;
};

export type RawAppConfig = Optional<{
	window_rules: Array<RawWindowRule>;
}>;

export type WindowRule = {
	command:
		| {
				type: "set";
				category: "translucent";
				value: number;
		  }
		| {
				type: "set";
				category: "title";
				value: boolean;
		  };
	match_process_name?: string;
	match_class_name?: string;
	match_title?: string;
};

export type AppConfig = {
	windowRules: Array<WindowRule>;
};

export const getAppConfig = async (): Promise<AppConfig> => {
	const appConfig = (await invoke<RawAppConfig>("get_app_config")) ?? [];
	const _rules = appConfig.window_rules ?? [];

	const windowRules = _rules
		.map((rule) => {
			const _command = rule.command ?? "";
			const elements = _command.split(" ");
			if (elements.length !== 3) return null;
			if (elements[0] !== "set") return null;
			const [type, category, _value] = elements;
			switch (category) {
				case "translucent": {
					const value = Number.parseInt(_value);
					if (Number.isNaN(value)) return null;
					if (value < 0 || value > 255) return null;
					return { ...rule, command: { type, category, value } };
				}

				case "title": {
					const value = !(_value === "false");
					return { ...rule, command: { type, category, value } };
				}
				default:
					return null;
			}
		})
		.filter((rule) => rule !== null) as WindowRule[];

	return {
		windowRules,
	};
};
