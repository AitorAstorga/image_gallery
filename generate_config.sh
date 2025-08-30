#!/bin/bash

# ==============================================================================
# Runtime Configuration Generator for Image Gallery
# ==============================================================================
# This script generates configuration from environment variables at container
# startup by modifying the HTML template and creating a JSON config file.
# ==============================================================================

# Use environment variables
GALLERY_TITLE=${GALLERY_TITLE}
GALLERY_HEADING=${GALLERY_HEADING}
GALLERY_DESCRIPTION=${GALLERY_DESCRIPTION}
VISIT_COUNTER_URL=${VISIT_COUNTER_URL}

# Function to escape HTML strings safely
escape_html_string() {
    echo "$1" | sed 's/&/\&amp;/g; s/</\&lt;/g; s/>/\&gt;/g'
}

# Function to escape JSON strings safely
escape_json_string() {
    echo "$1" | sed 's/\\/\\\\/g; s/"/\\"/g; s/\n/\\n/g; s/\r/\\r/g; s/\t/\\t/g'
}

echo "Generating gallery configuration from environment variables..."

# File paths
HTML_FILE="/usr/share/nginx/html/index.html"
CONFIG_JSON_FILE="/usr/share/nginx/html/static/gallery-config.json"

# Update HTML file title with environment variable
if [ -f "$HTML_FILE" ]; then
    # Replace {{GALLERY_TITLE}} placeholder with actual value
    ESCAPED_TITLE=$(escape_html_string "$GALLERY_TITLE")
    sed -i "s/{{GALLERY_TITLE}}/$ESCAPED_TITLE/g" "$HTML_FILE"
    echo "✅ Updated HTML title to: $GALLERY_TITLE"
else
    echo "⚠️  HTML file not found at: $HTML_FILE"
fi

# Generate JSON configuration for the Rust application
cat > "$CONFIG_JSON_FILE" << EOF
{
  "gallery_title": "$(escape_json_string "$GALLERY_TITLE")",
  "main_heading": "$(escape_json_string "$GALLERY_HEADING")",
  "description_html": "$(escape_json_string "$GALLERY_DESCRIPTION")",
  "visit_counter_url": "$(escape_json_string "$VISIT_COUNTER_URL")"
}
EOF

echo "✅ Configuration files generated at: $CONFIG_JSON_FILE"
echo "📋 Configuration summary:"
echo "   • Gallery Title: $GALLERY_TITLE"
echo "   • Main Heading: $GALLERY_HEADING"
echo "   • Description Length: ${#GALLERY_DESCRIPTION} characters"
echo "   • Visit Counter: ${VISIT_COUNTER_URL:0:50}..."
echo ""