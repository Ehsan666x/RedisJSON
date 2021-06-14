#[cfg(not(feature = "as-library"))]
use redis_module::{redis_command, redis_module};

use redis_module::native_types::RedisType;
use redis_module::raw::RedisModuleTypeMethods;
#[cfg(not(feature = "as-library"))]
use redis_module::{Context, RedisResult};

mod array_index;
mod backward;
pub mod commands;
pub mod error;
mod formatter;
pub mod manager;
mod nodevisitor;
pub mod redisjson;

use crate::redisjson::Format;
pub const REDIS_JSON_TYPE_VERSION: i32 = 3;

pub static REDIS_JSON_TYPE: RedisType = RedisType::new(
    "ReJSON-RL",
    REDIS_JSON_TYPE_VERSION,
    RedisModuleTypeMethods {
        version: redis_module::TYPE_METHOD_VERSION,

        rdb_load: Some(redisjson::type_methods::rdb_load),
        rdb_save: Some(redisjson::type_methods::rdb_save),
        aof_rewrite: None, // TODO add support
        free: Some(redisjson::type_methods::free),

        // Currently unused by Redis
        mem_usage: None,
        digest: None,

        // Auxiliary data (v2)
        aux_load: Some(redisjson::type_methods::aux_load),
        aux_save: None,
        aux_save_triggers: 0,

        free_effort: None,
        unlink: None,
        copy: None,
        defrag: None,
    },
);
/////////////////////////////////////////////////////

#[macro_export]
macro_rules! redis_json_module_create {(
        data_types: [
            $($data_type:ident),* $(,)*
        ],
        pre_command_function: $pre_command_function_expr:expr,
        get_manage: $get_manager_expr:expr,
        version: $version:expr,
        $(init: $init_func:ident,)* $(,)*
    ) => {

        ///
        /// JSON.DEL <key> [path]
        ///
        fn json_del(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_del(mngr, ctx, args),
                None => commands::command_json_del(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.GET <key>
        ///         [INDENT indentation-string]
        ///         [NEWLINE line-break-string]
        ///         [SPACE space-string]
        ///         [path ...]
        ///
        /// TODO add support for multi path
        fn json_get(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_get(mngr, ctx, args),
                None => commands::command_json_get(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.SET <key> <path> <json> [NX | XX | FORMAT <format>]
        ///
        fn json_set(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_set(mngr, ctx, args),
                None => commands::command_json_set(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.MGET <key> [key ...] <path>
        ///
        fn json_mget(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_mget(mngr, ctx, args),
                None => commands::command_json_mget(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.STRLEN <key> [path]
        ///
        fn json_str_len(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_str_len(mngr, ctx, args),
                None => commands::command_json_str_len(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.TYPE <key> [path]
        ///
        fn json_type(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_type(mngr, ctx, args),
                None => commands::command_json_type(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.NUMINCRBY <key> <path> <number>
        ///
        fn json_num_incrby(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_num_incrby(mngr, ctx, args),
                None => commands::command_json_num_incrby(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.NUMMULTBY <key> <path> <number>
        ///
        fn json_num_multby(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_num_multby(mngr, ctx, args),
                None => commands::command_json_num_multby(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.NUMPOWBY <key> <path> <number>
        ///
        fn json_num_powby(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_num_powby(mngr, ctx, args),
                None => commands::command_json_num_powby(manager::RedisJsonKeyManager, ctx, args),

            }
        }
        //
        /// JSON.TOGGLE <key> <path>
        fn json_bool_toggle(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_bool_toggle(mngr, ctx, args),
                None => commands::command_json_bool_toggle(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.STRAPPEND <key> [path] <json-string>
        ///
        fn json_str_append(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_str_append(mngr, ctx, args),
                None => commands::command_json_str_append(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.ARRAPPEND <key> <path> <json> [json ...]
        ///
        fn json_arr_append(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_arr_append(mngr, ctx, args),
                None => commands::command_json_arr_append(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.ARRINDEX <key> <path> <json-scalar> [start [stop]]
        ///
        /// scalar - number, string, Boolean (true or false), or null
        ///
        fn json_arr_index(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_arr_index(mngr, ctx, args),
                None => commands::command_json_arr_index(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.ARRINSERT <key> <path> <index> <json> [json ...]
        ///
        fn json_arr_insert(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_arr_insert(mngr, ctx, args),
                None => commands::command_json_arr_insert(manager::RedisJsonKeyManager, ctx, args),
            }
        }

        ///
        /// JSON.ARRLEN <key> [path]
        ///
        fn json_arr_len(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_arr_len(mngr, ctx, args),
                None => commands::command_json_arr_len(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.ARRPOP <key> [path [index]]
        ///
        fn json_arr_pop(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_arr_pop(mngr, ctx, args),
                None => commands::command_json_arr_pop(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.ARRTRIM <key> <path> <start> <stop>
        ///
        fn json_arr_trim(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_arr_trim(mngr, ctx, args),
                None => commands::command_json_arr_trim(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.OBJKEYS <key> [path]
        ///
        fn json_obj_keys(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_obj_keys(mngr, ctx, args),
                None => commands::command_json_obj_keys(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.OBJLEN <key> [path]
        ///
        fn json_obj_len(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_obj_len(mngr, ctx, args),
                None => commands::command_json_obj_len(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.CLEAR <key> [path ...]
        ///
        fn json_clear(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_clear(mngr, ctx, args),
                None => commands::command_json_clear(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.DEBUG <subcommand & arguments>
        ///
        /// subcommands:
        /// MEMORY <key> [path]
        /// HELP
        ///
        fn json_debug(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_debug(mngr, ctx, args),
                None => commands::command_json_debug(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        ///
        /// JSON.RESP <key> [path]
        ///
        fn json_resp(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_resp(mngr, ctx, args),
                None => commands::command_json_resp(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        fn json_cache_info(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_cache_info(mngr, ctx, args),
                None => commands::command_json_cache_info(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        fn json_cache_init(ctx: &Context, args: Vec<String>) -> RedisResult {
            $pre_command_function_expr(ctx, &args);
            let m = $get_manager_expr;
            match m {
                Some(mngr) => commands::command_json_cache_init(mngr, ctx, args),
                None => commands::command_json_cache_init(manager::RedisJsonKeyManager, ctx, args),

            }
        }

        redis_module! {
            name: "ReJSON",
            version: $version,
            data_types: [$($data_type,)*],
            $(
                init: $init_func,
            )*
            commands: [
                ["json.del", json_del, "write", 1,1,1],
                ["json.get", json_get, "readonly", 1,1,1],
                ["json.mget", json_mget, "readonly", 1,1,1],
                ["json.set", json_set, "write deny-oom", 1,1,1],
                ["json.type", json_type, "readonly", 1,1,1],
                ["json.numincrby", json_num_incrby, "write", 1,1,1],
                ["json.toggle", json_bool_toggle, "write deny-oom", 1,1,1],
                ["json.nummultby", json_num_multby, "write", 1,1,1],
                ["json.numpowby", json_num_powby, "write", 1,1,1],
                ["json.strappend", json_str_append, "write deny-oom", 1,1,1],
                ["json.strlen", json_str_len, "readonly", 1,1,1],
                ["json.arrappend", json_arr_append, "write deny-oom", 1,1,1],
                ["json.arrindex", json_arr_index, "readonly", 1,1,1],
                ["json.arrinsert", json_arr_insert, "write deny-oom", 1,1,1],
                ["json.arrlen", json_arr_len, "readonly", 1,1,1],
                ["json.arrpop", json_arr_pop, "write", 1,1,1],
                ["json.arrtrim", json_arr_trim, "write", 1,1,1],
                ["json.objkeys", json_obj_keys, "readonly", 1,1,1],
                ["json.objlen", json_obj_len, "readonly", 1,1,1],
                ["json.clear", json_clear, "write", 1,1,1],
                ["json.debug", json_debug, "readonly", 1,1,1],
                ["json.forget", json_del, "write", 1,1,1],
                ["json.resp", json_resp, "readonly", 1,1,1],
                ["json._cacheinfo", json_cache_info, "readonly", 1,1,1],
                ["json._cacheinit", json_cache_init, "write", 1,1,1],
            ],
        }
    }
}

#[cfg(not(feature = "as-library"))]
fn pre_command(_ctx: &Context, _args: &Vec<String>) {}

#[cfg(not(feature = "as-library"))]
redis_json_module_create! {
    data_types: [REDIS_JSON_TYPE],
    pre_command_function: pre_command,
    get_manage: Some(manager::RedisJsonKeyManager),
    version: 99_99_99,
}
