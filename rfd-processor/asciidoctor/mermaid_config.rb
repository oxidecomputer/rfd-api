# Passes a fixed mermaid configuration to every mmdc invocation made by
# asciidoctor-mermaid. The gem itself offers no way to set one.
#
# Mermaid 12 changed the defaults for unconfigured diagrams: the theme became
# "redux-color", the look became "neo", and the layout engine became ELK. The
# config in mermaid-config.json pins the pre-12 appearance ("default",
# "classic", dagre) so RFD diagrams render consistently.
#
# rfd-processor embeds this file and the JSON next to it, writes both to a temp
# directory at render time, and loads this file with `asciidoctor-pdf -r`.
require 'asciidoctor-mermaid/cli'

module Asciidoctor
  module Mermaid
    class Cli
      MERMAID_CONFIG_FILE = File.join(__dir__, 'mermaid-config.json')

      class << self
        alias_method :run_without_config, :run

        def run(content, *args)
          run_without_config(content, '--configFile', MERMAID_CONFIG_FILE, *args)
        end
      end
    end
  end
end
