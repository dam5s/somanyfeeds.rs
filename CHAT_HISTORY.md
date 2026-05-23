# Chat history

## Chat 1

 * Let's initialize an empty cargo project, the project name is somanyfeeds.
 * We are building a web app. Is there a popular, lightweight HTTP library people use? ideally something with routing and middleware support.
 * Let's build a hello world using Axum, the / endpoint should render an HTML page that says Hello World.
 * can we move the test to a separate file?
 * let's make sure the port can be set via an environment variable, we can default to 3000 if the PORT environment variable is not set.

## Chat 2

 * The tests were originally in the main.rs file, is this the generally accepted way to write tests in large Rust codebases?
 * Let's make sure the router and handlers are in a library separate from the app and have the tests in a tests directory.
 * is the name of lib.rs necessary? could we call it app.rs?
 * ok, how about integration_tests can we name that app_tests instead?
 * I want to have a codebase structure that allows for many applications and many components/libraries. Can we prepare for this?
   My codebase structure is usually something like:
   • apps/somanyfeeds_server
   • apps/somanyfeeds_cli
   • pkgs/feeds_processing
   • pkgs/database_support
 * what does resolver = "2" mean in the toplevel Cargo.toml?

## Chat 3

 * Let's create a public function for downloading content from a URL, it should return a result wrapping DownloadedContent which should have only one field of type string on it.
 * tests should follow the same pattern as somanyfeeds_server using the tests/ folder.
 * let's not expose reqwest on the public API, let's have a DownloadError type instead
 * do we need to derive(Debug)? let's remove it if it's unnecessary
 * Let's create and write tests for a function that can parse an RSS feed. For now, it should return a result that wraps a list of articles. An article only has a title field of type string. Let's convert the DownloadedContentError type and rename it to FeedsProcessingError and reuse that one.
   When writing a test we should use the content of the file mastodon.xml for validating we can parse correctly.
 * I forgot mastodon.xml was special and didn't have titles, can you update the parsing test to use damo.io.xml instead?
 * We brought in an rss library for parsing the rss feed. Is that library also able to parse atom feeds?
 * Hold on a second, do we need a feed parsing library? Is there some standard xml parsing library by chance? Only answer the question do not write code yet.
 * I am trying to minimize the number of dependencies, it looks like feed-rs is not actively maintained, I also only have a few feeds I need to support, there is a sample from each in the test_samples folder. Let's change to only use quick-xml, let's write tests for each of the provided xml files.
 * This is indeed very raw. Is there an integration between quick-xml and serde we could leverage? we would declare our own data types that map to the xml we want to support.
 * add a test for parsing bluesky.xml
 * let's make Article#title optional and not set it instead of having an empty string when there is no title.
 * Can we split this file in multiple parts? I'd like an rss file and an atom file for the data structures and parsing of each type of feed. I'd also like a downloads file for the download function and associated type.
 * let's also move the FeedsProcessingError to a file named error, and the parse_feed function and Article type into a file named feed_parsing

## Chat 4

 * Let's add the following fields to the Article type and let's make sure they are parsed correctly.
   link: Option<string> content: string date: DateTime<Utc>
 * can you update the tests for mastodon and bluesky to validate all the fields at least on one of the articles? the pattern in the test for damo.io.xml is good to follow.

## Chat 5

 * Is there an html templating library we can use for rendering html?
 * Let's use Askama. Maybe update the handler to read a quiery param named "name" and have the html say Hello {name} as an example usage.
 * there were some tests in app_tests.rs already please use this location for your tests instead of the new ones introduced in lib.rs

## Chat 6

 * I need to run work at regular interval while the server is running, let's say write something to the console for now every 30 seconds. Make that interval easy to change.
 * instead of reading the environment directly in the start function, let's wrap the start function into a type. Let's inject a structure of type WorkerSettings into it so that it's available to the start function. Building the WorkerSettings from the environment should only happen in main.rs
 * move the run_work function into the Worker type as well. then move the tests into the tests folder.

## Chat 7

 * Let's extract a function for loading a number from an environment variable.

## Chat 8

 * We're going to need a concept of Feed, let's create a type for it with name and url. Let's also create an in-memory FeedsRepository, it is initialized with a list of feeds, it has a find_all function that asynchronously returns all the feeds. Let's write that code in a feeds.rs file.
 * move it to the somanyfeeds_server app instead of the feeds_processing pkg
 * in main.rs we should instantiate a feeds repository before the worker, let's give it an example feed for now, we can use example.com for the url.
   Let's inject the repository into the worker, this way the worker will be able to load the feeds.
 * move the feeds tests to the tests folder

## Chat 9

 * Let's create an ArticlesRepository that can hold ArticleRecords. It is similar to the Feeds repository. It should be in a file named articles.rs next to the one for feeds. It should expose a function find_all, and a function replace_all to replace all the articles.
   The ArticleRecord should have the same fields as the Article type in the feeds_processing package, it should also have a feed_name field and feed_url field.
 * Update the Worker to do the following at each run:
   Find the feeds. For each feed process the feed and get articles. Log errors. Accumulate all articles successfully fetched into a single collection. Replace the articles in the ArticlesRepository at the end.
 * Can we avoid the nesting of matching of result types in the worker? Maybe by extracting a function that returns article records?
 * Let replace the handler for the root path. Now it should be able to list articles from the articles repository. It should sort the articles from the newest to the oldest, it should only keep the latest 30 articles.  Replace the hello template with an articles template that can display all the articles.

## Chat 10

 * Change the worker in worker.rs to save at most 20 articles per feed.If there is more than 20, keep the 20 most recent articles, using the date field.

## Chat 11

 * Look for tests that are in the same file as the implementation and move them into the tests directory instead.

## Chat 12

 * in lib.rs move the declared functions and the struct in a file named app.rs instead. lib.rs will only declare pub modules instead.

## Chat 13

 * we are duplicating version numbers in our Cargo.toml files is there a way to declare version numbers in the main Cargo.toml instead?

## Chat 14

 * we are using println in a few places, let's introduce proper logging. What options exist for logging?

## Chat 15

 * I'm trying to understand what are Arc best practices. Here I see that we assign it to the struct without cloning. When start() in invoked we do clone(), inside of run_work we do not clone but the caller is passing in a clone instead.
    My intuition would have been to just clone() on assignment to the struct.
    I'm more used to the objective-c style of retain release.

## Chat 16

 * The templates are currently stored in the somanyfeeds_server/templates directory, can we move those into somanyfeeds_server/resources/templates instead?
 * I added a reset.css and app.css in the resources/public folder. Let's have the web app able to serve static files in the public folder.
   Then let's add a reference to app.css in the articles.html template.
 * We are setting the public_path from the value of CARGO_MANIFEST_DIR, does that work when running the app after compilation? Or does that only work when using cargo run?
 * This is better, now instead of loading values from the environment inside of the router function, let's make a RouterSettings type and pass it as an argument to the router function. It should have a public_path field.
 * I noticed that the css link in articles.html is to /public/app.css, I want this to work directly with /app.css
    That is any file in the public path should be accessible at the root of the http server.
    GET /app.css should return the content of app.css inside the public directory.

## Chat 17

 * I want to format the date rendered in the HTML template in routes.rs.
     Let's rename the type ArticlesTemplate to ArticleListTemplate.
     Let's create a type ArticleView that will represent a single article and replace the type ArticleRecord in the articles field of ArticlesTemplate.
     The ArticleView will have the same fields as ArticleRecord except the date field should be of type string.
     The handler should format the date. The desired format should look like the following example: May 22 '26 @ 05:35 for 2026-05-22T05:35:00Z
 * The formatted date is in the UTC timezone, let's convert the date to the timezone America/Denver (Mountain Time) before formatting it. It's ok to hardcode that timezone.

## Chat 18

 * Let's remove truncation of the number of articles in the handler. We are now truncating per feed in the worker instead. All articles should be rendered.

## Chat 19

 * Let's cleanup the routes_tests, instead of having to instantiate a record and specifying all the fields each time we should make a default record available for tests. Tests should take that default record and only set fields that are relevant to the test, is that possible?
