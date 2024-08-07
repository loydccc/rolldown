// cSpell:disable

use std::collections::HashMap;

use crate::types::{binding_log::BindingLog, binding_log_level::BindingLogLevel};
use derivative::Derivative;
use napi::threadsafe_function::ThreadsafeFunction;
use napi_derive::napi;
use serde::Deserialize;

use self::{binding_input_item::BindingInputItem, binding_resolve_options::BindingResolveOptions};

use super::plugin::BindingPluginOrParallelJsPluginPlaceholder;

mod binding_input_item;
mod binding_resolve_options;
mod treeshake;

#[napi(object, object_to_js = false)]
#[derive(Deserialize, Default, Derivative)]
#[serde(rename_all = "camelCase")]
#[derivative(Debug)]
pub struct BindingInputOptions {
  // Not going to be supported
  // @deprecated Use the "inlineDynamicImports" output option instead.
  // inlineDynamicImports?: boolean;

  // acorn?: Record<string, unknown>;
  // acornInjectPlugins?: (() => unknown)[] | (() => unknown);
  // cache?: false | RollupCache;
  // context?: string;sssssssssss
  // experimentalCacheExpiry?: number;
  #[derivative(Debug = "ignore")]
  #[serde(skip_deserializing)]
  #[napi(
    ts_type = "undefined | ((source: string, importer: string | undefined, isResolved: boolean) => boolean)"
  )]
  pub external: Option<
    ThreadsafeFunction<(String, Option<String>, bool), bool, (String, Option<String>, bool), false>,
  >,
  pub input: Vec<BindingInputItem>,
  // makeAbsoluteExternalsRelative?: boolean | 'ifRelativeSource';
  // /** @deprecated Use the "manualChunks" output option instead. */
  // manualChunks?: ManualChunksOption;
  // maxParallelFileOps?: number;
  // /** @deprecated Use the "maxParallelFileOps" option instead. */
  // maxParallelFileReads?: number;
  // moduleContext?: ((id: string) => string | null | void) | { [id: string]: string };
  // onwarn?: WarningHandlerWithDefault;
  // perf?: boolean;
  #[serde(skip_deserializing)]
  #[napi(ts_type = "(BindingBuiltinPlugin | BindingPluginOptions | undefined)[]")]
  pub plugins: Vec<BindingPluginOrParallelJsPluginPlaceholder>,
  pub resolve: Option<BindingResolveOptions>,
  // preserveEntrySignatures?: PreserveEntrySignaturesOption;
  // /** @deprecated Use the "preserveModules" output option instead. */
  // preserveModules?: boolean;
  // pub preserve_symlinks: bool,
  pub shim_missing_exports: Option<bool>,
  // strictDeprecations?: boolean;
  // pub treeshake: Option<bool>,
  // watch?: WatcherOptions | false;
  #[napi(ts_type = "'node' | 'browser' | 'neutral'")]
  pub platform: Option<String>,
  #[serde(skip_deserializing)]
  pub log_level: Option<BindingLogLevel>,
  #[derivative(Debug = "ignore")]
  #[serde(skip_deserializing)]
  #[napi(ts_type = "(logLevel: 'debug' | 'warn' | 'info', log: BindingLog) => void")]
  pub on_log: BindingOnLog,
  // extra
  pub cwd: String,
  // pub builtins: BuiltinsOptions,
  pub treeshake: Option<treeshake::BindingTreeshake>,

  pub module_types: Option<HashMap<String, String>>,
  pub define: Option<Vec<(/* Target to be replaced */ String, /* Replacement */ String)>>,
}

pub type BindingOnLog =
  Option<ThreadsafeFunction<(String, BindingLog), (), (String, BindingLog), false>>;
