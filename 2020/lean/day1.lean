import .lovelib
import init.data.char.basic

namespace Day1

def search_helper : ℕ → ℕ → list ℕ → list (ℕ × ℕ)
| needle a (b :: bs) := 
    if a + b = needle
    then (a, b) :: search_helper needle a bs 
    else search_helper needle a bs
| _ _ [] := []

def search : ℕ → list ℕ → list (ℕ × ℕ)
| needle (n :: ns) := search_helper needle n ns ++ search needle ns
| _ [] := []

def search_spec : ℕ → set ℕ → set (ℕ × ℕ)
| needle ns := {ab | prod.fst ab ∈ ns ∧ prod.snd ab ∈ ns ∧ prod.fst ab + prod.snd ab = needle }

lemma search_search_spec (needle : ℕ) (ns : list ℕ) :
    set_of (λn, list.mem n (search needle ns)) = search_spec needle (set_of $ (λns n, list.mem n ns) ns) :=
begin
    induction' ns,
    case nil {
        ext1 ab,
        apply iff.intro,
        { 
            intro hab,
            tauto
        },
        {
            intro hab,
            cases' hab,
            tauto
        },
    },
    case cons {
        ext1 ab,
        apply iff.intro,
        { 
            intro hab,
            apply and.intro,
            {
                
            },
            apply and.intro,
            {
                sorry
            },
            { sorry }
            

        },
        { sorry }
    }
end

def search_helper_helper₂ : ℕ → ℕ → ℕ → list ℕ → list (ℕ × ℕ × ℕ)
| needle a b (c :: cs) :=
    if a + b + c = needle
    then (a, b, c) :: search_helper_helper₂ needle a b cs
    else search_helper_helper₂ needle a b cs
| _ _ _ [] := []

def search_helper₂ : ℕ → ℕ → list ℕ → list (ℕ × ℕ × ℕ)
| needle a (b :: bs) := search_helper_helper₂ needle a b bs ++ search_helper₂ needle a bs
| _ _ [] := []

def search₂ : ℕ → list ℕ → list (ℕ × ℕ × ℕ)
| needle (n :: ns) := search_helper₂ needle n ns ++ search₂ needle ns
| _ [] := []

def split_helper : char → list string → char → list string
| s (word :: words) c :=
    if s = c
    then ("" :: word :: words)
    else ((string.append word c.to_string) :: words)
| s [] c :=
    if s = c
    then ["", ""]
    else [c.to_string]

def split : string → char → list string
| xs s := string.fold [] (split_helper s) xs

def main : io unit := do 
    input ← io.fs.read_file "input1.txt",
    -- io.put_str_ln input.to_string,
    let words := list.reverse $ list.tail $ split input.to_string '\n',
    -- io.put_str_ln (words.to_string),
    let nums := functor.map string.to_nat words,
    -- io.put_str_ln nums.to_string,
    let answers := search 2020 nums,
    io.put_str_ln "answers:",
    io.put_str_ln answers.to_string,
    match list.nth answers 0 with
    | option.some (ab) := io.put_str_ln (prod.fst ab * prod.snd ab).repr
    | _ := pure ()
    end,
    let answers := search₂ 2020 nums,
    io.put_str_ln "answers₂:",
    io.put_str_ln answers.to_string,
    match list.nth answers 0 with
    | option.some (ab) := io.put_str_ln (prod.fst ab * (prod.fst $ prod.snd ab) * (prod.snd $ prod.snd ab)).repr
    | _ := pure ()
    end

end Day1

def main := Day1.main

#eval main