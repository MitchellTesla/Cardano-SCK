This style guide is mostly a copy of Johan Tibell's guide with some restructurization, elaboration on some topics and some additions. This style guide's aims are code beauty, readability and understandability.

Principles
The primary goal is readability/maintainability with performance as a close second.
Always use types to clarify intent and make invalid states unrepresentable.
Use type signatures as compiler checked documentation.
Break up large functions into smaller sub functions.
When modifying or adding code to existing code (either at the file, project or company level), try to match the style of the existing code.
General guide lines
Line Length
Maximum line length is 80 characters or 100 characters if necessary.

Modern screens have high definition and big width. But with some tiling managers and two terminals on one screen you are not able to see many characters on one line. On the other hand, restricting line size to a very small number like 80 leads to some crazy indentation despite the fact that shorter lines should force you to write well structured code. That's why 100 is a reasonable compromise.

Indentation
Do not vertically align code. Mainly because the need to change identifiers and re-aligning the other lines causes semantically null changes in the diff.
Tabs are illegal.
Use spaces for indenting.
Indent your code blocks with 2 spaces.
Indent the where keyword with one space to set it apart from the rest of the code and indent the definitions in a where clause with 1 space.
Some examples:

sayHello :: IO ()
sayHello = do
  name <- getLine
  putStrLn $ greeting name
 where
  greeting :: String -> String
  greeting name = "Hello, " ++ name ++ "!"

filter :: (a -> Bool) -> [a] -> [a]
filter _ [] = []
filter p (x:xs)
  | p x = x : filter p xs
  | otherwise = filter p xs
Blank Lines
One blank line between top-level definitions.
No blank lines between type signatures and function definitions.
Add one blank line between functions in a type class instance declaration if the function bodies are large.
You can add blank lines inside a big do block to separate logical parts of it.
You can also use blank lines to separate definitions inside where clause, if necessary.
Whitespace
Surround binary operators with a single space on either side. In case of currying add one space between the argument and the operation.

Use some tool to remove trailing spaces.

Naming convention
Casing:

lowerCamelCase for function and variable names.
UpperCamelCase for types.
Don't use short names like n, sk, f unless their meaning is clear from context (function name, types, other variables, etc.).

For readability reasons, don't capitalize all letters when using an abbreviation. For example, write HttpServer instead of HTTPServer. Exception: two or three letter abbreviations, e.g. IO, STM.

Records name conventions

If data type has only one constructor then this data type name should be the same as constructor name (also applies to newtype).

data User = User Int String
Field names for record data types

Choose a reasonable prefix is used such that we can ensure the field name will remain unique throughout the package. Use your discretion.

e.g

data MempoolEnv m blk = MempoolEnv
  { meLedger :: LedgerInterface m blk
  , meLedgerCfg :: LedgerConfig blk
  , meStateVar :: StrictTVar m (InternalState blk)
  , meTracer :: Tracer m (TraceEventMempool blk)
  }
Comments
Punctuation
Write proper sentences; start with a capital letter and use proper punctuation.

Top-Level Definitions
Comment top level functions (particularly exported functions) as much as possible. If it is not blindingly obvious what a function is doing, you should add a comment.
Always provide a type signature
Use Haddock syntax in the comments.
Comment every exported data type.
Function example:

-- | Send a message on a socket. The socket must be in a connected
-- state. Returns the number of bytes sent. Applications are
-- responsible for ensuring that all data has been sent.
send
  :: Socket      -- ^ Connected socket
  -> ByteString  -- ^ Data to send
  -> IO Int      -- ^ Bytes sent
For functions, the documentation should give enough information to apply the function without looking at its definition.
Record example:

-- | Bla bla bla.
data Person = Person
  { age :: !Int     -- ^ Age
  , name :: !String  -- ^ First name
  }
For fields that require longer comments, format them this way:

data Record = Record
  { -- | This is a very very very long comment that is split over
    -- multiple lines.
    field1 :: !Text

    -- | This is a second very very very long comment that is split
    -- over multiple lines.
  , field2 :: !Int
  }
End-of-Line Comments
Separate end-of-line comments from the code with 2 spaces. Align comments for data type definitions. Some examples:

data Parser =
  Parser
    !Int         -- ^ Current position
    !ByteString  -- ^ Remaining input

foo :: Int -> Int
foo n = salt * 32 + 9
  where
    salt = 453645243  -- Magic hash salt.
Links
Use in-line links economically. You are encouraged to add links for API names. It is not necessary to add links for all API names in a Haddock comment. We therefore recommend adding a link to an API name if:

The user might actually want to click on it for more information (in your opinion), and

Only for the first occurrence of each API name in the comment (don't bother repeating a link)

Top-down guideline
LANGUAGE extensions section
Write each LANGUAGE pragma on its own line, sort them alphabetically. Do not align by max width as future re-alignment when adding new pragmas will cause semantically null changes in the diff.

{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE Rank2Types #-}
{-# LANGUAGE TemplateHaskell #-}
Module name
Use singular when naming modules (e.g. use Data.Map and Data.ByteString.Internal instead of Data.Maps and Data.ByteString.Internals). Sometimes it's acceptable to use plural (e. g. Types, Instances).

Export Lists
Format export lists as follows:

module Data.Set
  ( Set
  , empty
  , singleton
  , member
  ) where
Some clarifications:

Use 2 spaces indentation for export list.
You can split export list into sections or just write all as single section.
It is strongly adviced to sort each section alpabetically. However, classes, data types and type aliases should be written before functions.
Imports
Imports should be grouped in the following order:

Import of custom prelude (for example cardano-prelude) if used.
External modules.
External IOHK modules.
Local library modules.
Local test modules
Put a blank line between each group of imports.

The imports in each group should be sorted alphabetically, by module name.

For external modules, use explicit or qualified imports.

For local modules, use implicit imports.

Always use explicit import lists or qualified imports.

Exception: cardano-prelude (import implicitly)
If the import is unqualified then put 11 spaces between the import keyword and the module name (i.e. length of qualified + 2).

Unqualified types (i.e. Map vs. M.Map) look pretty good and not so ugly. Prefer two-line imports for such standard containers.

import           Data.Map (Map)
import qualified Data.Map as Map
Data Declarations
Align the constructors in a data type definition. Example:

data HttpException
  = InvalidStatusCode Int
  | MissingContentHeader
Format records as follows:

data Person = Person
  { firstName :: !String  -- ^ First name
  , lastName :: !String  -- ^ Last name
  , age :: !Int     -- ^ Age
  } deriving (Eq, Show)
Type classes in deriving section should be always surrounded by parentheses.

WARNING: try to avoid aggressive autoderiving. Deriving instances can slowdown compilation (stated here: http://www.stephendiehl.com/posts/production.html)

Deriving instances of Read/Show/Data/Generic for largely recursive ADTs can sometimes lead to quadratic memory behavior when the nesting gets deep.

Function declaration
All functions must have type signatures.

Specialize function type signature for concrete types if you're using this function with only one type for each argument. Otherwise you should use more polymorphic version. Compiler can optimize specialized functions better and meaning of this function may be clearer. Use this rule unless you are the library creator and want your library to be abstract as possible.

It is allowed to omit parentheses for only one type class constraint.

If a function type signature is very long then place some of the types on its own line.

Example :

putValueInState
  :: MonadIO m
  => UserState -> Maybe Int -> AppConfig
  -> Integer -> (Int -> m ()) -> m ()
List Declarations
Align the elements in the list. Example:

exceptions =
  [ InvalidStatusCode
  , MissingContentHeader
  , InternalServerError
  ]
If-then-else clauses
Generally, guards and pattern matches should be preferred over if-then-else clauses, where possible. Short cases should usually be put on a single line (when line length allows it).

Case expressions
The alternatives in a case expression can be indented using either of the two following styles:

foobar =
  case something of
    Just j -> foo
    Nothing -> bar
or as

foobar = case something of
           Just j -> foo
           Nothing -> bar
but not

foobar = case something of
    Just j  -> foo
    Nothing -> bar
Subexpressions should always be indented to the right of their parent expression.

Dealing with laziness
By default, use strict data types and lazy functions.

Data types
Constructor fields should be strict, unless there's an explicit reason to make them lazy. This helps to avoid many common pitfalls caused by too much laziness and reduces the number of brain cycles the programmer has to spend thinking about evaluation order.

-- Good
data Point = Point
  { pointX :: !Double  -- ^ X coordinate
  , pointY :: !Double  -- ^ Y coordinate
  }
-- Bad
data Point = Point
  { pointX :: Double  -- ^ X coordinate
  , pointY :: Double  -- ^ Y coordinate
  }
Misc
Point-free style
Avoid over-using point-free style. Again, use your discretion.

For example, this is hard to read:

-- Bad:
f = (g .) . h
Whereas this is fine:

unFooBar :: Foo -> Baz
unFooBar = Baz . unBar . unFoo
The point-free version needs to have a clarity benefit but we realize that this is vague so use your discretion.

Cabal file formatting
Modules & libraries
Modules and libraries should go in alphabetical order inside corresponding sections. You can put blank lines between groups in each section.

Warnings
Code should be compilable with -Wall without warnings.

Default extensions
The only allowed default extension is: NoImplicitPrelude All other extensions need to be put in the relevant module.

Language Extensions to avoid
Avoid RecordWildCards - use NamedFieldPuns instead. RecordWildCards makes it difficult to discern where record accessor functions come from, especially when used with external libraries. This results in time wasted searching for the accessors. NamedFieldPuns forces you to be more explicit and is a better alternative.

Avoid ViewPatterns - ViewPatterns obfuscates what could have been readable code and incurs more cognitive overhead.

OPTIONS_GHC pragma to avoid
-Wno-incomplete-uni-patterns - This a bad idea. GHC will not warn you if you have not explicitly handled all possible cases in your function definitions. This opens up the possibility of a runtime error.

-Wno-partial-fields - This will allow you to define a partial record accessor which opens you up to the possibility of a runtime error.
