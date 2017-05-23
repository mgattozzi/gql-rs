struct SourceCharacter(char); // Need to clamp restrictions on this
struct UnicodeBOM;
enum WhiteSpace {
    HorizontalTab,
    Space
}
enum LineTerminator {
    NewLine,
    CarriageReturn,
    /// Carriage Return New Line \r\n
    CRNL
}
struct Comma;
// Should start with a # for a comment
struct Comment(Vec<CommentChar>);
struct CommentChar(SourceCharacter); // but not LineTerminator

struct Name(String); // Need to clamp restrictions on this

enum Token {
    Punc(Punctuator),
    Name(Name),
    IntValue(i64),
    FloatValue(f64),
    StringValue(String)
}

// TODO Do From/Into for all of these.
enum Ignored {
    UBOM(UnicodeBOM),
    WS(WhiteSpace),
    LT(LineTerminator),
    CMNT(Comment),
    COM(Comma),
}

enum Punctuator {
    ExclamationPoint, // !
    Dollar,           // $
    LParen,           // (
    RParen,           // )
    TriDot,           // ...
    Colon,            // :
    Equal,            // =
    At,               // @
    LBracket,         // [
    RBracket,         // ]
    LBrace,           // {
    RBrace,           // }
    Pipe              // |
}

struct Document(Vec<Definition>);

enum Definition {
    OpDef(OperationDefintion),
    FragDefinition
}

struct OperationDefintion {
    op_type: Option<OperationType>, // It's not depending?
    name: Option<Name>,
    var_def: Option<VariableDefinitions>,
    dirs: Option<Directives>,
    sel_set: SelectionSet
}

enum OperationType {
    Query,
    Mutation,
    Subscription,
}
// temp
struct VariableDefinitions;
struct Directives;
struct SelectionSet(Vec<Selection>);
struct Arguments(Vec<Argument>);
struct Argument {
    name: Name,
    value: Value,
}
struct Field {
    alias: Option<Alias>,
    name: Name,
    arguments: Option<Arguments>,
    directives: Option<Directives>,
}

struct Alias(Name);
struct FragmentSpread {
    fragment_name: FragmentName,
    directives: Option<Directives>,
}
struct FragmentDefinition {
    fragment_name: FragmentName,
    type_condition: TypeCondition,
    directives: Option<Directives>,
    selection_set: SelectionSet
}
struct FragmentName(Name);
enum Selection {
    Field(Field),
    FragmentSpread(FragmentSpread),
    InlineFragment(InlineFragment),
}
struct InlineFragment {
    type_condition: Option<TypeCondition>,
    directives: Option<Directives>,
    selection_set: SelectionSet,
}
struct TypeCondition(NamedType);

enum Value {
    Variable,
    IntValue(IntValue),
    FloatValue,
    StringValue,
    BooleanValue,
    NullValue,
    EnumValue,
    ListValue,
    ObjectValue,
}

struct IntValue(IntegerPart);

struct IntegerPart {
    negative_sign: Option<NegativeSign>,
    int_part: IntPart,
}

enum IntPart {
    Zero,
    NonZero((NonZeroDigit, Option<Vec<Digit>>)),
}

struct NegativeSign;
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
enum NonZeroDigit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}


