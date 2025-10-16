(comment) @comment
(string) @string
(number) @number

(module_identifier) @label

(enum_constraint (named_number (identifier) @enum))

"::=" @operator
"," @punctuation.delimiter
; ".." @punctuation.delimiter
; "|" @punctuation.delimiter
"{" @punctuation.bracket
"}" @punctuation.bracket
"(" @punctuation.bracket
")" @punctuation.bracket
"BEGIN" @punctuation.bracket
"END" @punctuation.bracket

(assignment (_ (identifier) @variable))
(object_identifier_value (identifier) @variable)
(objects_clause (identifier) @variable)

"INTEGER" @type
"BITS" @type
"OCTET STRING" @type
(basetype) @type
(sequence_of_type (identifier) @type)

"DESCRIPTION" @property
"INDEX" @property
"MAX-ACCESS" @property
"MIN-ACCESS" @property
"ORGANIZATION" @property
"PRODUCT-RELEASE" @property
"REVISION" @property
"STATUS" @property
"SYNTAX" @property
"UNITS" @property
"LAST-UPDATED" @property
"CONTACT-INFO" @property
"DEFVAL" @property

"ACCESS" @keyword
"AGENT-CAPABILITIES" @keyword
"ALL" @keyword
"AUGMENTS" @keyword
"DEFINITIONS" @keyword
"DISPLAY-HINT" @keyword
"ENTERPRISE" @keyword
"EXPORTS" @keyword
"FROM" @keyword
"GROUP" @keyword
"IMPORTS" @keyword
"IMPLIED" @keyword
"INCLUDES" @keyword
"MANDATORY-GROUPS" @keyword
"MODULE" @keyword
"MODULE-COMPLIANCE" @keyword
"MODULE-IDENTITY" @keyword
"NOTIFICATION-GROUP" @keyword
"NOTIFICATION-TYPE" @keyword
"OBJECT" @keyword
"OBJECT IDENTIFIER" @keyword
"OBJECT-GROUP" @keyword
"OBJECT-IDENTITY" @keyword
"OBJECT-TYPE" @keyword
"OBJECTS" @keyword
"OF" @keyword
"REFERENCE" @keyword
"SEQUENCE" @keyword
"SUPPORTS" @keyword
"TEXTUAL-CONVENTION" @keyword
"TRAP-TYPE" @keyword
"VARIABLES" @keyword
"VARIATION" @keyword
"WRITE-SYNTAX" @keyword

"accessible-for-notify" @variant
"current" @variant
"deprecated" @variant
"mandatory" @variant
"not-accessible" @variant
"obsolete" @variant
"optional" @variant
"read-create" @variant
"read-only" @variant
"read-write" @variant
"write-only" @variant
