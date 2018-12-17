use ammonia::Builder;

// TODO: white list all classes

/// Clean html from cross-site scripting, layout breaking, and clickjacking
pub fn sanitize(input: &str) -> String {
    Builder::default()
        .generic_attributes(hashset!["class"])
        .clean(&input)
        .to_string()
}

mod test {

    #[test]
    fn clean_light_card() {
        use super::sanitize;
        let src = r#"<h2>Example </h2><p><br></p><pre class="ql-syntax" spellcheck="false"><span class="hljs-keyword">use</span> ammonia::Builder;

<span class="hljs-comment">/// Clean html from cross-site scripting, layout breaking, and clickjacking</span>
<span class="hljs-keyword">pub</span> <span class="hljs-function"><span class="hljs-keyword">fn</span> <span class="hljs-title">sanitize</span></span>(input: &amp;<span class="hljs-built_in">str</span>) -&gt; <span class="hljs-built_in">String</span> {
        Builder::<span class="hljs-keyword">default</span>()
                .generic_attributes(hashset![<span class="hljs-string">"class"</span>])
                .clean(&amp;input)
                .to_string()
}
</pre>"#;
        let cleaned = "<h2>Example </h2><p><br></p><pre class=\"ql-syntax\"><span class=\"hljs-keyword\">use</span> ammonia::Builder;\n\n<span class=\"hljs-comment\">/// Clean html from cross-site scripting, layout breaking, and clickjacking</span>\n<span class=\"hljs-keyword\">pub</span> <span class=\"hljs-function\"><span class=\"hljs-keyword\">fn</span> <span class=\"hljs-title\">sanitize</span></span>(input: &amp;<span class=\"hljs-built_in\">str</span>) -&gt; <span class=\"hljs-built_in\">String</span> {\n        Builder::<span class=\"hljs-keyword\">default</span>()\n                .generic_attributes(hashset![<span class=\"hljs-string\">\"class\"</span>])\n                .clean(&amp;input)\n                .to_string()\n}\n</pre>";

        assert_eq!(sanitize(&src), cleaned);
    }
}
