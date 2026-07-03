export interface ImportResult {
  imported: number
  skipped: number
}

export type Familiarity = 'unknown' | 'half' | 'known'
export type QuizType = 'choice' | 'fill' | 'recall'
export type ResultType = 'correct' | 'wrong' | 'skipped'

export interface Stats {
  total: number
  unknown: number
  half: number
  known: number
}

export interface Word {
  id: number
  word: string
  source?: string
  meaning?: string
}

export interface ChoiceQuiz {
  type: 'choice'
  word: string
  correct: string
  options: string[]
}

export interface FillQuiz {
  type: 'fill'
  word: string
  hint: string
  first: string
  last: string
}

export interface RecallQuiz {
  type: 'recall'
  word: string
  answer: string
}

export type Quiz = ChoiceQuiz | FillQuiz | RecallQuiz

export interface StudyResultPayload {
  word: string
  quiz_type: QuizType
  result: ResultType
  familiarity_after: Familiarity
}
