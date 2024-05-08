use feed_rs::model::Entry;
use cel_interpreter::{Context, Value};
use crate::config::common::MyCelProgram;
use crate::utils::string::strip_html;
use chrono::{DateTime, FixedOffset};

pub fn filter<'a>(entries: &'a Vec<&Entry>, includes: &Vec<&MyCelProgram>, excludes: &Vec<&MyCelProgram>, now: &DateTime<FixedOffset>) -> Vec<&'a Entry> {
    let mut result : Vec<&Entry> = Vec::new();

    for entry in entries {
        if filter_predicate(entry, includes, excludes, now) {
            result.push(entry);
        }
    }

    result
}

fn filter_predicate(entry: &Entry, includes: &Vec<&MyCelProgram>, excludes: &Vec<&MyCelProgram>, now: &DateTime<FixedOffset>) -> bool {
    let mut use_it = true;

    let context = get_cel_context_for(entry, now);

    if use_it == true && excludes.len() > 0 {
        if does_any_expression_match(&context, excludes) {
            use_it = false
        }
    }

    if use_it == true && includes.len() > 0 {
        if !does_any_expression_match(&context, includes) {
            use_it = false
        }
    }

    return use_it
}

fn does_any_expression_match(context: &Context, programs: &Vec<&MyCelProgram>) -> bool {
    for program in programs {
        match program.execute(&context).unwrap() {
            Value::Bool(b) => if b == true { return true; },
            v => panic!("CEL did not return a boolean result: {:?}", v),
        }
    }
    false
}

pub fn get_cel_context_for<'a>(entry: &'a Entry, now: &DateTime<FixedOffset>) -> Context<'a> {
    let ref id = entry.id;
    let title = entry.title.as_ref().map(|v| &v.content);
    let content = entry.content.as_ref().map(|a| a.body.as_ref()).flatten().map(|b| strip_html(b));
    let summary = entry.summary.as_ref().map(|a| &a.content).map(|b| strip_html(b));
    let updated = entry.updated.as_ref().map(|t| t.fixed_offset());
    let published = entry.published.as_ref().map(|t| t.fixed_offset());
    let categories: Vec<&String> = entry.categories.iter().map(|c| &c.term).collect();
    let links: Vec<&String> = entry.links.iter().map(|l| &l.href).collect();

    let mut context = Context::default();
    context.add_variable("id", id).unwrap();
    context.add_variable("title", title).unwrap();
    context.add_variable("content", content).unwrap();
    context.add_variable("summary", summary).unwrap();
    context.add_variable("categories", categories).unwrap();
    context.add_variable("links", links).unwrap();
    context.add_variable("updated", match updated {
        Some(t) => Value::Timestamp(t),
        None => Value::Null,
    }).unwrap();
    context.add_variable("published", match published {
        Some(t) => Value::Timestamp(t),
        None => Value::Null,
    }).unwrap();

    let now = Value::Timestamp(*now);
    context.add_variable("now", now).unwrap();

    // {
    //     println!("======");
    //     println!("id: {:?}", context.get_variable("id"));
    //     println!("title: {:?}", context.get_variable("title"));
    //     println!("content: {:?}", context.get_variable("content"));
    //     println!("summary: {:?}", context.get_variable("summary"));
    //     println!("categories: {:?}", context.get_variable("categories"));
    //     println!("links: {:?}", context.get_variable("links"));
    //     println!("updated: {:?}", context.get_variable("updated"));
    //     println!("published: {:?}", context.get_variable("published"));
    //     println!("now: {:?}", context.get_variable("now"));
    //     println!("======");
    // }

    context
}

pub fn debug_cel_context(context: &Context) {
    println!("======");
    println!("id: {:?}", context.get_variable("id"));
    println!("title: {:?}", context.get_variable("title"));
    println!("content: {:?}", context.get_variable("content"));
    println!("summary: {:?}", context.get_variable("summary"));
    println!("categories: {:?}", context.get_variable("categories"));
    println!("links: {:?}", context.get_variable("links"));
    println!("updated: {:?}", context.get_variable("updated"));
    println!("published: {:?}", context.get_variable("published"));
    println!("now: {:?}", context.get_variable("now"));
    println!("======");
}