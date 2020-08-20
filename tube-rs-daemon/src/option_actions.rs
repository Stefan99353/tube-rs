use diesel::prelude::*;

use crate::db_models::{DefaultOption, InsertDefaultOption, InsertOptionGroup, OptionGroup};

pub fn get_option_groups(
    conn: &SqliteConnection,
) -> Result<Vec<(OptionGroup, Vec<DefaultOption>)>, diesel::result::Error> {
    use crate::schema::{default_options, option_groups};

    let groups: Vec<OptionGroup> = option_groups::table.load(conn)?;

    let mut result: Vec<(OptionGroup, Vec<DefaultOption>)> = vec![];

    for group in groups {
        let options: Vec<DefaultOption> = default_options::table
            .filter(default_options::group_id.eq(group.id))
            .load(conn)?;

        result.push((group, options));
    }

    Ok(result)
}

pub fn get_option_group_by_id(
    group_id: i32,
    conn: &SqliteConnection,
) -> Result<Option<OptionGroup>, diesel::result::Error> {
    use crate::schema::option_groups;

    let result: Option<OptionGroup> = option_groups::table
        .filter(option_groups::id.eq(group_id))
        .first::<OptionGroup>(conn)
        .optional()?;

    Ok(result)
}

pub fn add_option_group(
    new_option_group: OptionGroup,
    new_default_options: Vec<DefaultOption>,
    conn: &SqliteConnection,
) -> Result<Option<OptionGroup>, diesel::result::Error> {
    use crate::schema::{default_options, option_groups};

    let result: Option<OptionGroup> = option_groups::table
        .filter(option_groups::name.eq(new_option_group.name.to_string()))
        .first::<OptionGroup>(conn)
        .optional()?;

    match result {
        Some(_) => {
            // Already in DB
            Ok(None)
        }
        None => {
            // Insert OptionGroup in DB

            let ins_option_group = InsertOptionGroup {
                name: new_option_group.name.to_string(),
            };

            diesel::insert_into(option_groups::table)
                .values(&ins_option_group)
                .execute(conn)?;

            // Get id of inserted OptionGroup
            let group: OptionGroup = option_groups::table
                .filter(option_groups::name.eq(new_option_group.name))
                .first::<OptionGroup>(conn)?;

            let mut ins_default_options: Vec<InsertDefaultOption> = vec![];

            for default_option in new_default_options {
                ins_default_options.push(InsertDefaultOption {
                    flag: default_option.flag,
                    val: default_option.val,
                    group_id: group.id,
                })
            }

            diesel::insert_into(default_options::table)
                .values(&ins_default_options)
                .execute(conn)?;

            Ok(Some(group))
        }
    }
}

pub fn update_option_group(
    option_group: OptionGroup,
    conn: &SqliteConnection,
) -> Result<Option<OptionGroup>, diesel::result::Error> {
    use crate::schema::option_groups;

    let result: Option<OptionGroup> = option_groups::table
        .filter(option_groups::id.eq(option_group.id))
        .first::<OptionGroup>(conn)
        .optional()?;

    match result {
        None => Ok(None),
        Some(old_group) => {
            diesel::update(option_groups::table.find(old_group.id))
                .set((option_groups::name.eq(option_group.name.to_string()),))
                .execute(conn)?;

            Ok(Some(option_group))
        }
    }
}

pub fn remove_option_group(
    group_id: i32,
    conn: &SqliteConnection,
) -> Result<bool, diesel::result::Error> {
    use crate::schema::{default_options, option_groups};

    let result: Option<OptionGroup> = option_groups::table
        .filter(option_groups::id.eq(group_id))
        .first::<OptionGroup>(conn)
        .optional()?;

    match result {
        None => Ok(false),
        Some(group) => {
            diesel::delete(default_options::table.filter(default_options::group_id.eq(group_id)))
                .execute(conn)?;

            diesel::delete(option_groups::table.filter(option_groups::id.eq(group.id)))
                .execute(conn)?;

            Ok(true)
        }
    }
}

pub fn get_default_options_by_id(
    group_id: i32,
    conn: &SqliteConnection,
) -> Result<Option<Vec<DefaultOption>>, diesel::result::Error> {
    use crate::schema::{default_options, option_groups};

    let result: Option<OptionGroup> = option_groups::table
        .filter(option_groups::id.eq(group_id))
        .first::<OptionGroup>(conn)
        .optional()?;

    match result {
        None => Ok(None),
        Some(group) => {
            let default_options: Vec<DefaultOption> = default_options::table
                .filter(default_options::group_id.eq(group.id))
                .load::<DefaultOption>(conn)?;

            Ok(Some(default_options))
        }
    }
}

pub fn add_default_option(
    new_default_option: InsertDefaultOption,
    conn: &SqliteConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::default_options;

    diesel::insert_into(default_options::table)
        .values(&new_default_option)
        .execute(conn)?;

    Ok(())
}

pub fn remove_default_option(
    option_id: i32,
    conn: &SqliteConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::default_options::dsl::*;

    diesel::delete(default_options.filter(id.eq(option_id))).execute(conn)?;

    Ok(())
}

pub fn update_default_option(
    default_option: DefaultOption,
    conn: &SqliteConnection,
) -> Result<(), diesel::result::Error> {
    use crate::schema::default_options::dsl::*;

    diesel::update(default_options.find(default_option.id))
        .set((flag.eq(default_option.flag), val.eq(default_option.val)))
        .execute(conn)?;

    Ok(())
}
