use std::collections::HashMap;

pub struct TemplateRegistry {
    templates: HashMap<String, String>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        
        // Photo/Image templates
        templates.insert("photo-date".to_string(), "%N_%D.%E".to_string());
        templates.insert("photo-counter".to_string(), "photo_%C3.%E".to_string());
        templates.insert("photo-datetime".to_string(), "%N_%FD_%FH.%E".to_string());
        
        // Document templates
        templates.insert("doc-date".to_string(), "%N_%D.%E".to_string());
        templates.insert("doc-counter".to_string(), "document_%C2.%E".to_string());
        
        // Lowercase templates
        templates.insert("lowercase".to_string(), "%L%N.%E".to_string());
        templates.insert("lowercase-name".to_string(), "%N%L.%E".to_string());
        
        // Uppercase templates
        templates.insert("uppercase".to_string(), "%U%N.%E".to_string());
        templates.insert("uppercase-name".to_string(), "%N%U.%E".to_string());
        
        // Title case templates
        templates.insert("title-case".to_string(), "%T%N.%E".to_string());
        templates.insert("title-case-name".to_string(), "%N%T.%E".to_string());
        
        // Parent directory templates
        templates.insert("parent-prefix".to_string(), "%P_%N.%E".to_string());
        templates.insert("parent-suffix".to_string(), "%N_%P.%E".to_string());
        
        // Counter templates
        templates.insert("counter-2".to_string(), "%C2.%E".to_string());
        templates.insert("counter-3".to_string(), "%C3.%E".to_string());
        templates.insert("counter-4".to_string(), "%C4.%E".to_string());
        templates.insert("counter-prefix".to_string(), "%C3_%N.%E".to_string());
        templates.insert("counter-suffix".to_string(), "%N_%C3.%E".to_string());
        
        // Date/time templates
        templates.insert("date-suffix".to_string(), "%N_%D.%E".to_string());
        templates.insert("date-prefix".to_string(), "%D_%N.%E".to_string());
        templates.insert("datetime-suffix".to_string(), "%N_%D_%H.%E".to_string());
        
        // Cleanup templates
        templates.insert("trim-spaces".to_string(), "%M%N.%E".to_string());
        templates.insert("underscore-to-dash".to_string(), "%N%R/_/-.%E".to_string());
        templates.insert("dash-to-underscore".to_string(), "%N%R/-/_.%E".to_string());
        
        Self { templates }
    }
    
    pub fn get(&self, name: &str) -> Option<&String> {
        self.templates.get(name)
    }
    
    pub fn list(&self) -> Vec<(&String, &String)> {
        let mut items: Vec<_> = self.templates.iter().collect();
        items.sort_by_key(|(k, _)| *k);
        items
    }
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

