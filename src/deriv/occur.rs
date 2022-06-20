use crate::deriv::so::{ SyntacticObject };

/// Occurrence.
/// 
/// From Definition 17 in C&S 2016, p. 52:
/// 
/// >B *occurs* in A at position P iff $P = \\langle A, \ldots, B \\rangle$. We also say B has an occurrence in A at position P (written $B\_p$).
/// 
/// We call P the position of B.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Occurrence<'a> {
    pub position: Vec<&'a SyntacticObject>,
}



impl Occurrence<'_> {
    /// Check if an occurrence is valid, defined in Definition 16.
    /// 
    /// From Definition 16 in C&S 2016, p. 51:
    /// 
    /// >The *position* of $\\textrm{SO}\_n$ in $\\textrm{SO}\_1$ is a *path*, a sequence of syntactic objects $\\langle \\textrm{SO}\_1, \\textrm{SO}\_2, \\ldots, \\textrm{SO}\_n \\rangle$ where for all $0 < i < n$, $\\textrm{SO}\_{i+1} \\in \\textrm{SO}\_i$.
    pub fn check(&self) -> bool {
        //  The position cannot be an empty path.
        (!self.position.is_empty()) &&
        self.position.windows(2)
            .all(|so_pair| {
                let so1 = so_pair[0];
                let so2 = so_pair[1];
                so1.immediately_contains(so2)
            })
    }

    /// Immediate containment for occurrences.
    /// 
    /// From Definition 18 in C&S 2016, p. 52:
    /// 
    /// >Let A, B and C be syntactic objects, then, in C, occurrence $B\_P$ *immediately contains* occurrence $A\_{P'}$ (for paths $P, P'$ in C) iff $P = \\langle X\_1, \ldots, X\_n \\rangle$ and $P' = \\langle X\_1, \ldots, X\_n, X\_{n+1} \\rangle$.
    pub fn immediately_contains<'b>(&self, other: &Occurrence<'b>) -> bool {
        //  Assuming both self and other are valid occurrences... (TODO)

        (self.position.len() + 1 == other.position.len()) &&
        {
            let len = self.position.len();
            self.position[..] == other.position[..len]
        }
    }

    /// Containment for occurrences.
    /// 
    /// Extending the defintion of containment for syntactic objects (see [`SyntacticObject::contains`]), containment for occurrences is defined as the transitive closure of immediate containment.
    pub fn contains<'b>(&self, other: &Occurrence<'b>) -> bool {
        //  Assuming both self and other are valid occurrences... (TODO)

        (self.position.len() < other.position.len()) &&
        {
            let len = self.position.len();
            self.position[..] == other.position[..len]
        }
    }

    /// Sisterhood for occurrences.
    /// 
    /// From Definition 20, C&S 2016, p. 53.
    /// 
    /// >Let $A$, $B$, $C$ be syntactic objects (where $A \\neq B$), then in $C$, $A_P$ is a *sister* of $B_{P'}$ iff $P = \\langle X\_1, \ldots, X\_{n-1}, X\_n \\rangle$ (where $X\_{n-1} = C$ and $X\_n = A$) and $P' = \\langle X\_1, \ldots, X\_{n-1}, X'\_n \\rangle$ (where $X'\_n = B$).
    pub fn sisters_with(&self, other: &Occurrence, under: &SyntacticObject) -> bool {
        //  Assuming both self and other are valid occurrences... (TODO)

        (self != other) &&
        (self.position.len() >= 2) &&
        (self.position.len() == other.position.len()) &&
        {
            let len = self.position.len() - 1;
            (self.position[..] == other.position[..]) &&
            (self.position[len - 1] == under)
        }
    }

    /// C-command for occurrences.
    /// 
    /// From Definition 22, C&S 2016, p. 53.
    /// 
    /// >In $D$, $A_P$ *c-commands* $B_{P'}$ iff there is an occurrence $C_{P''}$ such that:
    /// >
    /// >1.  $C_{P''}$ is a sister of $A_P$ in $D$, and
    /// >2.  either $B_{P'} = C_{P''}$ or $C_{P''}$ contains $B_{P'}$, in $D$.
    pub fn c_commands(&self, other: &Occurrence, under: &SyntacticObject) -> bool {
        //  `under` is D in the definition.
        under.iter_contained_as_occ()
            .any(|occ| {
                //  `occ` is C in the defnition.
                occ.sisters_with(self, under) &&
                (other == occ || occ.contains(other))
            })
    }

    /// Asymmetric c-command.
    /// 
    /// Something like:
    /// 
    /// >$A_P$ *asymmetrically c-commands* $B_{P'}$ iff $A_P$ c-commands $B_{P'}$ and $A_P$ and $B_{P'}$ are not sisters.
    pub fn asymmetrically_c_commands(&self, other: &Occurrence, under: &SyntacticObject) -> bool {
        (!self.sisters_with(other, under)) &&
        self.c_commands(other, under)
    }
}